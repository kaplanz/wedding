#![allow(clippy::unused_async)]

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

add_impl! { About Home Dashboard Login Registry Rsvp Travel }

#[derive(Template)]
#[template(path = "about.html")]
pub struct About;

impl About {
    fn new() -> Self {
        Self
    }

    pub async fn get() -> impl IntoResponse {
        Self::new()
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home;

impl Home {
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

#[derive(Default, Template)]
#[template(path = "login.html")]
pub struct Login {
    msg: Option<String>,
}

impl Login {
    fn new() -> Self {
        Self::default()
    }

    pub async fn get() -> impl IntoResponse {
        Self::new()
    }

    pub async fn msg(msg: String) -> impl IntoResponse {
        Self { msg: Some(msg) }
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

#[derive(Template)]
#[template(path = "travel.html")]
pub struct Travel;

impl Travel {
    fn new() -> Self {
        Self
    }

    pub async fn get() -> impl IntoResponse {
        Self::new()
    }
}
