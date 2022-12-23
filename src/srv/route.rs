use std::net::SocketAddr;

use askama::Template;
use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use log::{error, trace};

use super::auth::{self, AuthContext};
use crate::db::guest::Reply;
use crate::db::{self, Database};
use crate::user::User;

pub async fn index() -> impl IntoResponse {
    Index::get().await
}

pub async fn login(auth: AuthContext) -> impl IntoResponse {
    match auth.current_user {
        Some(_) => Redirect::to("/rsvp").into_response(),
        None => Login::get().await.into_response(),
    }
}

pub async fn auth(
    State(db): State<Database>,
    auth: AuthContext,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(mut user): Form<User>,
) -> impl IntoResponse {
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
    // Redirect onwards to rsvp
    Redirect::to("/rsvp")
}

pub async fn logout(auth: AuthContext) -> impl IntoResponse {
    auth::logout(auth).await;
    Redirect::to("/")
}

pub async fn registry() -> impl IntoResponse {
    Registry::get().await
}

pub async fn rsvp(auth: AuthContext) -> impl IntoResponse {
    Rsvp::get(auth.current_user.unwrap().clone()).await
}

pub async fn reply(
    State(mut db): State<Database>,
    auth: AuthContext,
    Form(reply): Form<Reply>,
) -> impl IntoResponse {
    let user = auth.current_user.unwrap();
    db.update(&user, reply).unwrap();
    match db.write() {
        Ok(_) | Err(db::Error::Path) => (),
        Err(err) => error!("{err}"),
    };
    Redirect::to("/")
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

impl Index {
    fn new() -> Self {
        Self
    }

    async fn get() -> impl IntoResponse {
        Self::new()
    }
}

impl IntoResponse for Index {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("500: Failed to render template: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "login.html")]
struct Login;

impl Login {
    fn new() -> Self {
        Self
    }

    async fn get() -> impl IntoResponse {
        Self::new()
    }
}

impl IntoResponse for Login {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("500: Failed to render template: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "registry.html")]
struct Registry;

impl Registry {
    fn new() -> Self {
        Self
    }

    async fn get() -> impl IntoResponse {
        Self::new()
    }
}

impl IntoResponse for Registry {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("500: Failed to render template: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "rsvp.html")]
struct Rsvp {
    user: User,
}

impl Rsvp {
    fn new(user: User) -> Self {
        Self { user }
    }

    async fn get(user: User) -> impl IntoResponse {
        Self::new(user)
    }
}

impl IntoResponse for Rsvp {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("500: Failed to render template: {err}"),
            )
                .into_response(),
        }
    }
}
