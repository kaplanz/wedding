use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{ConnectInfo, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use log::{debug, error, trace, warn};
use serde::Deserialize;
use tokio::sync::RwLock;

use self::page::{Dashboard, Home, Login, Registry, Rsvp};
use super::auth::{self, AuthContext};
use super::Error;
use crate::db::guest::Reply;
use crate::db::{self, Database, Ident};
use crate::user::User;

mod page;

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
    auth: AuthContext,
) -> impl IntoResponse {
    // Redirect to the login if no user authenticated
    let Some(user) = auth.current_user.clone() else {
        return Redirect::to("/login").into_response();
    };
    // Acquire database as a reader
    let db = db.read().await;
    // Get all the guests in this user's group
    // TODO: Handle errors better (don't just unwrap)
    let guests = db
        .group(&user.ident)
        .unwrap()
        .iter()
        .map(|ident| db.guest(ident).unwrap())
        .cloned()
        .collect();
    // Present dashbaord page
    Dashboard::get(user, guests).await.into_response()
}

pub async fn login(auth: AuthContext) -> impl IntoResponse {
    match auth.current_user {
        // Redirect if already logged in
        Some(_) => Redirect::to("/dashboard").into_response(),
        // Present login page
        None => Login::get().await.into_response(),
    }
}

pub async fn auth(
    State(db): State<Arc<RwLock<Database>>>,
    auth: AuthContext,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(mut user): Form<User>,
) -> impl IntoResponse {
    // Sanitize user input
    trace!("attempt: `{user}`, from: {addr}");
    user.sanitize();
    // Acquire database as a reader
    let db = db.read().await;
    // Query the database using provided credentials
    let Some(ident) = db.query(&user).cloned() else {
        // User not found
        warn!("reject: `{user}`, from: {addr}");
        // Redirect back on failuer
        return Login::msg(
            format!("Hmm, we couldn't find a login for: {user}")
        ).await.into_response();
    };
    // Update user identifier
    user.ident = ident;
    // Authenticate user
    auth::login(auth, user).await;
    // Redirect onwards to RSVP
    Redirect::to("/dashboard").into_response()
}

pub async fn logout(auth: AuthContext) -> impl IntoResponse {
    // Perform logout for the user
    auth::logout(auth).await;
    // Redirect to the homepage
    Redirect::to("/login")
}

pub async fn registry() -> impl IntoResponse {
    // Present regsitry page
    Registry::get().await
}

pub async fn rsvp(
    State(db): State<Arc<RwLock<Database>>>,
    auth: AuthContext,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(action): Query<Action>,
) -> impl IntoResponse {
    // Redirect to the login if no user authenticated
    let Some(user) = auth.current_user.clone() else {
        return Redirect::to("/login").into_response();
    };
    // Present to the user if no guest supplied
    let guest = action.guest.unwrap_or(user.ident);
    // Acquire database as a reader
    let db = db.read().await;
    // Confirm this user is in the requested guest's group
    let group = db.group(&user.ident).unwrap();
    if !group.contains(&guest) {
        warn!("unauthorized: `{user}`, from: {addr}");
        return Error::e401().into_response();
    }
    // Extract the guest to RSVP
    let guest = db.guest(&guest).unwrap().clone();
    // Present RSVP page
    Rsvp::get(guest).await.into_response()
}

pub async fn reply(
    State(db): State<Arc<RwLock<Database>>>,
    auth: AuthContext,
    Query(action): Query<Action>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(reply): Form<Reply>,
) -> impl IntoResponse {
    // Do nothing if not logged in
    let Some(user) = auth.current_user else {
        // User not found, return status code
        return StatusCode::UNAUTHORIZED.into_response();
    };
    // Reply for the user if no guest supplied
    let guest = action.guest.unwrap_or(user.ident);
    // Acquire database as a writer
    let mut db = db.write().await;
    // Confirm this user is in the requested guest's group
    let group = db.group(&user.ident).unwrap();
    if !group.contains(&guest) {
        warn!("unauthorized: `{user}`, from: {addr}");
        return Error::e401().into_response();
    }
    // Update this user's reply
    // TODO: Handle errors better (don't just unwrap)
    debug!("reply: `{user}`, for: {}", guest);
    db.update(&guest, reply).unwrap();
    // Save the database to a file (optional)
    // TODO: Should this be done async?
    match db.write() {
        Ok(_) | Err(db::Error::Path) => (),
        Err(err) => error!("{err}"),
    };
    // Redirect to the homepage
    Redirect::to("/dashboard").into_response()
}
