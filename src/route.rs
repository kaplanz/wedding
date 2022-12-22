use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;

use crate::auth::{self, AuthContext};
use crate::db::Database;
use crate::guest::Reply;
use crate::user::User;

pub async fn index() -> impl IntoResponse {
    Index::get().await
}

pub async fn login() -> impl IntoResponse {
    Login::get().await
}

pub async fn auth(auth: AuthContext, Form(user): Form<User>) -> impl IntoResponse {
    auth::login(auth, user).await;
    Redirect::to("/rsvp")
}

pub async fn logout(auth: AuthContext) -> impl IntoResponse {
    auth::logout(auth).await;
    Redirect::to("/")
}

pub async fn rsvp(auth: AuthContext) -> impl IntoResponse {
    Rsvp::get(auth.current_user.unwrap().clone()).await
}

pub async fn update(State(mut db): State<Database>, Form(reply): Form<Reply>) -> impl IntoResponse {
    db.update(0, reply).unwrap();
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
