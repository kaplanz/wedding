use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

use crate::user::User;

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl IntoResponse for $t {
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
    )*)
}

add_impl! { Index Login Registry Rsvp }

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

impl Index {
    fn new() -> Self {
        Self
    }

    pub async fn get() -> impl IntoResponse {
        Self::new()
    }
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login;

impl Login {
    fn new() -> Self {
        Self
    }

    pub async fn get() -> impl IntoResponse {
        Self::new()
    }
}

#[derive(Template)]
#[template(path = "registry.html")]
pub struct Registry;

impl Registry {
    fn new() -> Self {
        Self
    }

    pub async fn get() -> impl IntoResponse {
        Self::new()
    }
}

#[derive(Template)]
#[template(path = "rsvp.html")]
pub struct Rsvp {
    user: User,
}

impl Rsvp {
    fn new(user: User) -> Self {
        Self { user }
    }

    pub async fn get(user: User) -> impl IntoResponse {
        Self::new(user)
    }
}
