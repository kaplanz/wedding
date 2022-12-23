use std::io;

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub async fn e404() -> impl IntoResponse {
    Error::new(StatusCode::NOT_FOUND, "Page not found :(".to_string())
}

pub async fn e500(err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}"))
}

#[derive(Template)]
#[template(path = "error.html")]
struct Error {
    code: StatusCode,
    msg: String,
}

impl Error {
    fn new(code: StatusCode, msg: String) -> Self {
        Self { code, msg }
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
