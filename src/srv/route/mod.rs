use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{self, ConnectInfo, FromRequest, FromRequestParts, State};
use axum::response::{IntoResponse, Redirect};
use log::{error, trace, warn};
use serde::Deserialize;
use tokio::sync::RwLock;

use self::page::{Dashboard, Home, Login, Registry, Rsvp, Travel};
use super::{auth, Error};
use crate::db::guest::Reply;
use crate::db::{self, Database, Ident};
use crate::user::User;

mod page;

#[derive(FromRequest)]
#[from_request(via(extract::Form), rejection(Error))]
pub struct Form<T>(T);

#[derive(FromRequestParts)]
#[from_request(via(extract::Query), rejection(Error))]
pub struct Query<T>(T);

#[derive(Debug, Deserialize)]
pub struct Action {
    guest: Option<Ident>,
}

pub async fn home() -> impl IntoResponse {
    // Present homepage
    Home::get().await
}

pub async fn dashboard(
    State(db): State<Arc<RwLock<Database>>>,
    auth: auth::Context,
) -> impl IntoResponse {
    // Redirect to the login if no user authenticated
    let Some(user) = auth.current_user.clone() else {
        return Err(Redirect::to("/login").into_response());
    };
    // Acquire database as a reader
    let db = db.read().await;
    // Get all the guests in this user's group
    let guests = db
        .group(&user.ident)
        .map_err(|err| Error::e500(err).into_response())?
        .iter()
        .map(|ident| {
            db.guest(ident)
                .ok_or_else(|| Error::e500(db::Error::Guest).into_response())
                .cloned()
        })
        .collect::<Result<_, _>>()?;
    // Present dashboard page
    Ok(Dashboard::get(user, guests).await)
}

pub async fn login(auth: auth::Context) -> impl IntoResponse {
    match auth.current_user {
        // Redirect if already logged in
        Some(_) => Ok(Redirect::to("/dashboard")),
        // Present login page
        None => Err(Login::get().await),
    }
}

pub async fn auth(
    State(db): State<Arc<RwLock<Database>>>,
    auth: auth::Context,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(mut user): Form<User>,
) -> impl IntoResponse {
    // Sanitize user input
    trace!("attempt: `{user}`, from: {addr}");
    user.sanitize();
    // Acquire database as a reader
    let db = db.read().await;
    // Query the database using provided credentials
    let Some(ident) = db.query(&user).copied() else {
        // User not found
        warn!("reject: `{user}`, from: {addr}");
        // Return with error message on failure
        return Err(Login::msg(
            format!("Hmm, we couldn't find a login for: {user}")
        ).await);
    };
    // Update user identifier
    user.ident = ident;
    // Authenticate user
    auth::login(auth, user).await;
    // Redirect onwards to RSVP
    Ok(Redirect::to("/dashboard"))
}

pub async fn logout(auth: auth::Context) -> impl IntoResponse {
    // Perform logout for the user
    auth::logout(auth).await;
    // Redirect to the homepage
    Redirect::to("/login")
}

pub async fn registry() -> impl IntoResponse {
    // Present registry page
    Registry::get().await
}

pub async fn rsvp(
    State(db): State<Arc<RwLock<Database>>>,
    auth: auth::Context,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(action): Query<Action>,
) -> impl IntoResponse {
    // Redirect to the login if no user authenticated
    let Some(user) = auth.current_user.clone() else {
        return Err(Redirect::to("/login").into_response());
    };
    // Present to the user if no guest supplied
    let guest = action.guest.unwrap_or(user.ident);
    // Acquire database as a reader
    let db = db.read().await;
    // Confirm this user is in the requested guest's group
    let group = db
        .group(&user.ident)
        .map_err(|err| Error::e500(err).into_response())?;
    if !group.contains(&guest) {
        // Guest not in user's group
        warn!("unauthorized: `{user}`, from: {addr}");
        // Present error page on failure
        return Err(Error::e401().into_response());
    }
    // Extract the guest to RSVP
    let guest = db
        .guest(&guest)
        .ok_or_else(|| Error::e500(db::Error::Guest).into_response())?
        .clone();
    // Present RSVP page
    Ok(Rsvp::get(guest).await)
}

pub async fn reply(
    State(db): State<Arc<RwLock<Database>>>,
    auth: auth::Context,
    Query(action): Query<Action>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(mut reply): Form<Reply>,
) -> impl IntoResponse {
    // Do nothing if not logged in
    let Some(user) = auth.current_user else {
        // User not found, return status code
        return Err(Error::e401());
    };
    // Reply for the user if no guest supplied
    let guest = action.guest.unwrap_or(user.ident);
    // Acquire database as a writer
    let mut db = db.write().await;
    // Confirm this user is in the requested guest's group
    let group = db.group(&user.ident).map_err(Error::e500)?;
    if !group.contains(&guest) {
        // Guest not in user's group
        warn!("unauthorized: `{user}`, from: {addr}");
        // Present error page on failure
        return Err(Error::e401());
    }
    // Update this user's reply
    trace!(
        "reply: `{user}`, for: `{}`",
        db.guest(&guest)
            .ok_or_else(|| Error::e500(db::Error::Guest))?
            .user()
    );
    reply.validate();
    db.update(&guest, reply).map_err(Error::e500)?;
    // Save the database to a file (optional)
    // TODO: Should this be done async?
    match db.write() {
        Ok(_) | Err(db::Error::Path) => (),
        Err(err) => error!("{err}"),
    };
    // Redirect to the homepage
    Ok(Redirect::to("/dashboard"))
}

pub async fn travel() -> impl IntoResponse {
    // Present travel page
    Travel::get().await
}
