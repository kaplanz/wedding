use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use log::{error, trace};
use uuid::Uuid;

use self::page::{Dashboard, Index, Login, Registry, Rsvp};
use super::auth::{self, AuthContext};
use super::error;
use crate::db::guest::Reply;
use crate::db::{self, Database};
use crate::user::User;

mod page;

pub async fn index() -> impl IntoResponse {
    // Present index page
    Index::get().await
}

pub async fn dashboard(State(db): State<Database>, auth: AuthContext) -> impl IntoResponse {
    // Redirect to the login if no user authenticated
    let Some(user) = auth.current_user.clone() else {
        return Redirect::to("/login").into_response();
    };
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
    State(db): State<Database>,
    auth: AuthContext,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(mut user): Form<User>,
) -> impl IntoResponse {
    // Query the database using provided credentials
    trace!("attempt: `{user}`, from: {addr}");
    let Some(ident) = db.query(&user).cloned() else {
        // User not found
        trace!("reject: `{user}`");
        // Redirect back on failuer
        return Redirect::to("/login");
    };
    // Update user identifier
    user.ident = ident;
    // Authenticate user
    auth::login(auth, user).await;
    // Redirect onwards to RSVP
    Redirect::to("/dashboard")
}

pub async fn logout(auth: AuthContext) -> impl IntoResponse {
    // Perform logout for the user
    auth::logout(auth).await;
    // Redirect to the homepage
    Redirect::to("/")
}

pub async fn registry() -> impl IntoResponse {
    // Present regsitry page
    Registry::get().await
}

pub async fn rsvp(
    State(db): State<Database>,
    auth: AuthContext,
    Path(guest): Path<Uuid>,
) -> impl IntoResponse {
    // Redirect to the login if no user authenticated
    let Some(user) = auth.current_user.clone() else {
        return Redirect::to("/login").into_response();
    };
    // Confirm this user is in the requested guest's group
    let group = db.group(&user.ident).unwrap();
    if !group.contains(&guest) {
        trace!("unauthorized: `{user}`");
        return error::e401().await.into_response();
    }
    // Extract the guest to RSVP
    let guest = db.guest(&guest).unwrap().clone();
    // Present RSVP page
    Rsvp::get(guest).await.into_response()
}

pub async fn reply(
    State(mut db): State<Database>,
    auth: AuthContext,
    Path(guest): Path<Uuid>,
    Form(reply): Form<Reply>,
) -> impl IntoResponse {
    // Do nothing if not logged in
    let Some(user) = auth.current_user else {
        // User not found, return status code
        return StatusCode::UNAUTHORIZED.into_response();
    };
    // Confirm this user is in the requested guest's group
    let group = db.group(&user.ident).unwrap();
    if !group.contains(&guest) {
        trace!("unauthorized: `{user}`");
        return error::e401().await.into_response();
    }
    // Update this user's reply
    // TODO: Handle errors better (don't just unwrap)
    trace!("reply: `{user}`, for: `{}`", guest);
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
