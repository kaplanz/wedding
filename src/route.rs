use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use axum_login::AuthUser;

use crate::auth::{self, AuthContext};
use crate::user::User;

pub async fn index() -> impl IntoResponse {
    Index::get().await
}

pub async fn login() -> impl IntoResponse {
    Login::get().await
}

pub async fn auth(auth: AuthContext, Form(user): Form<User>) -> impl IntoResponse {
    auth::login(auth, user).await;
    Redirect::to("/")
}

pub async fn logout(auth: AuthContext) -> impl IntoResponse {
    auth::logout(auth).await;
    Redirect::to("/")
}

pub async fn rsvp(auth: AuthContext) -> impl IntoResponse {
    Rsvp::get(&auth.current_user.unwrap()).await
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
#[template(path = "rsvp.html")]
struct Rsvp {
    guest: String,
}

impl Rsvp {
    fn new(guest: String) -> Self {
        Self { guest }
    }

    async fn get(user: &User) -> impl IntoResponse {
        Self::new(user.get_id())
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
