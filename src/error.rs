use axum::{http::Response, response::IntoResponse};
use reqwest::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    LoginFail,

    // - Model errors
    TicketDeleteFailIdNotFound {
        id: u64,
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response<axum::body::Body> {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}