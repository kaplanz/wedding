use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

use crate::db::guest::Guest;
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

add_impl! { Index Dashboard Login Registry Rsvp }

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
#[template(path = "dashboard.html")]
pub struct Dashboard {
    user: User,
    guests: Vec<Guest>,
}

impl Dashboard {
    fn new(user: User, guests: Vec<Guest>) -> Self {
        Self { user, guests }
    }

    pub async fn get(user: User, guests: Vec<Guest>) -> impl IntoResponse {
        Self::new(user, guests)
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
    guest: Guest,
}

impl Rsvp {
    fn new(guest: Guest) -> Self {
        Self { guest }
    }

    pub async fn get(guest: Guest) -> impl IntoResponse {
        Self::new(guest)
    }
}
