use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use log::{error, trace};

use self::page::{Index, Login, Registry, Rsvp};
use super::auth::{self, AuthContext};
use crate::db::guest::Reply;
use crate::db::{self, Database};
use crate::user::User;

mod page;

pub async fn index() -> impl IntoResponse {
    Index::get().await
}

pub async fn login(auth: AuthContext) -> impl IntoResponse {
    match auth.current_user {
        // Redirect if already logged in
        Some(_) => Redirect::to("/rsvp").into_response(),
        // Perform the login
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
    let Some(ident) = db.query(&user) else {
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
    Redirect::to("/rsvp")
}

pub async fn logout(auth: AuthContext) -> impl IntoResponse {
    // Perform logout for the user
    auth::logout(auth).await;
    // Redirect to the homepage
    Redirect::to("/")
}

pub async fn registry() -> impl IntoResponse {
    Registry::get().await
}

pub async fn rsvp(auth: AuthContext) -> impl IntoResponse {
    // Redirect to the login if no user authenticated
    let Some(user) = auth.current_user.clone() else {
        return Redirect::to("/login").into_response();
    };
    // Present RSVP form
    Rsvp::get(user).await.into_response()
}

pub async fn reply(
    State(mut db): State<Database>,
    auth: AuthContext,
    Form(reply): Form<Reply>,
) -> impl IntoResponse {
    // Do nothing if not logged in
    let Some(user) = auth.current_user else {
        // User not found, return status code
        return StatusCode::UNAUTHORIZED.into_response();
    };
    // Update this user's reply
    db.update(&user, reply).unwrap();
    // Save the database to a file (optional)
    // TODO: Should this be done async?
    match db.write() {
        Ok(_) | Err(db::Error::Path) => (),
        Err(err) => error!("{err}"),
    };
    // Redirect to the homepage
    Redirect::to("/").into_response()
}
