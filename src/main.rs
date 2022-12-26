use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::handler::HandlerWithoutStateExt;
use axum::routing::{get, get_service};
use axum::Router;
use axum_login::axum_sessions::{async_session, SessionLayer};
use axum_login::{memory_store, AuthLayer, AuthUser};
use axum_server::tls_rustls::RustlsConfig;
use clap::{Parser, ValueHint};
use color_eyre::eyre::Result;
use log::{info, warn};
use rand::Rng;
use tokio::sync::RwLock;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod db;
mod srv;
mod user;

use crate::db::Database;
use crate::srv::{error, route};

/// Hannah & Zakhary's wedding server.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Port to listen for connections.
    #[arg(short, long)]
    #[arg(default_value_t = 3000)]
    #[arg(env = "PORT")]
    port: u16,

    /// Path to input guestlist.
    #[arg(value_hint = ValueHint::FilePath)]
    guests: Option<PathBuf>,

    /// Path to output guestlist.
    #[arg(short, long)]
    #[arg(value_hint = ValueHint::FilePath)]
    out: Option<PathBuf>,

    /// Certificate file for TLS.
    #[arg(long)]
    cert: Option<PathBuf>,

    /// Key file for TLS.
    #[arg(long)]
    key: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Install panic and error report handlers
    color_eyre::install()?;
    // Initialize tracing
    tracing_subscriber::fmt::init();
    // Parse args
    let args = Args::parse();

    // Extract TLS certificate and key
    let tls = CertKey::try_from((args.cert, args.key)).ok();

    // Initialize database
    let mut db = match &args.guests {
        Some(path) => Database::try_from(path.as_path())?,
        None => {
            warn!("no guestlist provided, login will not be possible");
            Database::default()
        }
    };
    db.path = args.out; // set optional output file
    info!("loaded {} guests", db.len());
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
    // Wrap dataabase layer
    let db = Arc::new(RwLock::new(db));

    // Build our application with a route
    let app = Router::new()
        .route("/", get(route::index))
        .route("/dashboard", get(route::dashboard))
        .route("/login", get(route::login).post(route::auth))
        .route("/logout", get(route::logout))
        .route("/registry", get(route::registry))
        .route("/rsvp", get(route::rsvp).post(route::reply))
        .fallback_service(
            get_service(
                ServeDir::new("www").not_found_service(
                    error::e404
                        .into_service()
                        .map_err(|err| -> io::Error { match err {} }),
                ),
            )
            .handle_error(error::e500),
        )
        .layer(TraceLayer::new_for_http())
        .layer(auth)
        .layer(session)
        .with_state(db);

    // Run it
    let addr = SocketAddr::from(([0; 8], args.port));
    match tls {
        Some(tls) => {
            // Prepare TLS config
            let config = RustlsConfig::from_pem_file(tls.cert, tls.key)
                .await
                .unwrap();
            // Serve the app
            info!("listening on https://{addr}");
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();
        }
        None => {
            // Serve the app
            info!("listening on http://{addr}");
            axum_server::bind(addr)
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();
        }
    }

    Ok(())
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
