use askama::Template;
use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use thiserror::Error;

pub async fn e404() -> impl IntoResponse {
    Error::e404()
}

pub async fn e500<E: std::error::Error>(err: E) -> impl IntoResponse {
    Error::e500(err)
}

#[derive(Debug, Error, Template)]
#[template(path = "error.html")]
pub struct Error {
    code: StatusCode,
    msg: String,
}

impl Error {
    fn new(code: StatusCode, msg: String) -> Self {
        Self { code, msg }
    }

    pub fn e400() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "Something went wrong...".to_string(),
        )
    }

    pub fn e401() -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "You don't have access to that!".to_string(),
        )
    }

    pub fn e404() -> Self {
        Self::new(StatusCode::NOT_FOUND, "Page not found :(".to_string())
    }

    pub fn e500<E: std::error::Error>(err: E) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, format!("{err}"))
    }
}

impl From<FormRejection> for Error {
    fn from(_: FormRejection) -> Self {
        Self::e400()
    }
}

impl IntoResponse for Error {
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
