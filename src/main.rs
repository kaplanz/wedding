use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::handler::HandlerWithoutStateExt;
use axum::http::Request;
use axum::routing::{get, get_service};
use axum::Router;
use axum_login::axum_sessions::{async_session, SessionLayer};
use axum_login::{memory_store, AuthLayer, AuthUser};
use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use clap::{Parser, ValueHint};
use color_eyre::eyre::{Context, Result};
use log::{debug, error, info, trace, warn};
use rand::Rng;
use tokio::signal;
use tokio::signal::unix::SignalKind;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod db;
mod srv;
mod user;

use crate::db::Database;
use crate::srv::route::Incoming;
use crate::srv::{error, route};

/// Hannah & Zakhary's wedding server.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to input guestlist.
    #[arg(value_hint = ValueHint::FilePath)]
    guests: Option<PathBuf>,

    /// Path to output guestlist.
    #[arg(short, long)]
    #[arg(value_hint = ValueHint::FilePath)]
    out: Option<PathBuf>,

    /// Path to log output file.
    #[arg(long)]
    #[arg(value_hint = ValueHint::FilePath)]
    log: Option<PathBuf>,

    /// Port to listen for connections.
    #[arg(short, long)]
    #[arg(default_value_t = 3000)]
    #[arg(env = "PORT")]
    port: u16,

    /// Directory root for serving files.
    #[arg(short, long)]
    #[arg(default_value = "www")]
    #[arg(value_hint = ValueHint::FilePath)]
    root: PathBuf,

    /// Certificate file for TLS.
    #[arg(long)]
    cert: Option<PathBuf>,

    /// Key file for TLS.
    #[arg(long)]
    key: Option<PathBuf>,

    /// Disable RSVP system.
    #[arg(long)]
    lock: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Install panic and error report handlers
    color_eyre::install()?;
    // Parse args
    let args = Args::parse();

    // Initialize tracing
    let filter = EnvFilter::from_default_env(); // use default filter
    let out = tracing_subscriber::fmt::layer(); // log to stdout
                                                // produce logfile
    let log = if let Some(path) = args.log {
        let file = File::options()
            .append(true)
            .create(true)
            .write(true)
            .open(path)
            .context("failed to open log file")?;
        let log = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(file);
        Some(log)
    } else {
        None
    };
    tracing_subscriber::registry()
        .with(filter)
        .with(out)
        .with(log)
        .init();

    // Extract TLS certificate and key
    let tls = CertKey::try_from((args.cert, args.key)).ok();
    if let Some(tls) = &tls {
        debug!(
            "tls: cert: `{}`, key: `{}`",
            tls.cert.display(),
            tls.key.display()
        );
    }

    // Initialize database
    let mut db = match &args.guests {
        Some(path) => {
            Database::try_from(path.as_path()).context("failed to initialize database")?
        }
        None => {
            // Initialize empty database
            warn!("no guestlist provided, login will not be possible");
            Database::default()
        }
    };
    info!("loaded {} guests", db.len());
    db.locked = args.lock;
    if db.locked {
        warn!("database is locked");
    }
    // Set (optional) database write path
    db.path = args.out;
    if let Some(path) = &db.path {
        debug!("database: output path: `{}`", path.display());
    }

    // Initialize tracing layer
    let trace = TraceLayer::new_for_http().on_request(|req: &Request<_>, _: &_| {
        // Collect incoming info
        let info = Incoming::new(req);
        // Log the request
        trace!("{info}");
    });

    // Initialize session layer
    let secret: [u8; 64] = rand::thread_rng().gen();
    let session = SessionLayer::new(async_session::MemoryStore::new(), &secret).with_secure(false);

    // Initialize auth layer
    let users = db
        .iter()
        .map(|user| (user.get_id(), user.clone()))
        .collect();
    let memory = Arc::new(RwLock::new(users));
    let store = memory_store::MemoryStore::new(&memory);
    let auth = AuthLayer::new(store, &secret);

    // Wrap database layer
    let db = Arc::new(RwLock::new(db));

    // Build our application with routes
    debug!("directory root: `{}`", &args.root.display());
    let app = Router::new()
        .route("/", get(route::home))
        .route("/about", get(route::about))
        .route("/dashboard", get(route::dashboard))
        .route("/login", get(route::login).post(route::auth))
        .route("/logout", get(route::logout))
        .route("/registry", get(route::registry))
        .route("/rsvp", get(route::rsvp).post(route::reply))
        .route("/travel", get(route::travel))
        .layer(trace)
        .fallback_service(get_service(
            ServeDir::new(args.root).not_found_service(error::e404.into_service()),
        ))
        .layer(auth)
        .layer(session)
        .with_state(db);

    // Create a handle for the server
    let handle = Handle::new();

    // Spawn a task to gracefully shutdown server
    tokio::spawn(signal(handle.clone()));

    // Run it
    let addr = SocketAddr::from(([0; 8], args.port));
    if let Some(tls) = tls {
        // Prepare TLS config
        let config = RustlsConfig::from_pem_file(tls.cert, tls.key)
            .await
            .unwrap();
        // Serve the app
        info!("listening on {addr}");
        axum_server::bind_rustls(addr, config)
            .handle(handle)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    } else {
        // Serve the app
        info!("listening on {addr}");
        axum_server::bind(addr)
            .handle(handle)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }

    Ok(())
}

async fn signal(handle: Handle) {
    // Prepare signal handlers
    let mut sigterm =
        signal::unix::signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                // Signal the server to gracefully shutdown
                warn!("commencing graceful shutdown");
                info!("nofifying {} connections", handle.connection_count());
                handle.graceful_shutdown(Some(Duration::from_secs(1)));
            },
            _ = sigterm.recv() => {
                // Signal the server to terminate
                error!("terminating");
                handle.shutdown();
            },
        }
    }
}

#[derive(Debug)]
struct CertKey {
    cert: PathBuf,
    key: PathBuf,
}

impl TryFrom<(Option<PathBuf>, Option<PathBuf>)> for CertKey {
    type Error = ();

    fn try_from((cert, key): (Option<PathBuf>, Option<PathBuf>)) -> Result<Self, Self::Error> {
        match (cert, key) {
            (Some(cert), Some(key)) => Ok(Self { cert, key }),
            (Some(cert), None) => {
                warn!(
                    "missing TLS key file, ignoring certificate: `{}`",
                    cert.display()
                );
                Err(())
            }
            (None, Some(key)) => {
                warn!(
                    "missing TLS certificate file, ignoring key: `{}`",
                    key.display()
                );
                Err(())
            }
            (None, None) => Err(()),
        }
    }
}
