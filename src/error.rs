use axum::{http::Response, response::IntoResponse};
use reqwest::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
#[allow(dead_code)]
pub enum Error {
    LoginFail,

    // - Model errors
    TicketDeleteFailIdNotFound {
        id: u64,
    },

    // - Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt
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
        
        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn clien_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // - Auth errors
            Self::AuthFailNoAuthTokenCookie
                | Self::AuthFailTokenWrongFormat
                | Self::AuthFailCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            // - Model errors
            Self::TicketDeleteFailIdNotFound { id } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // - Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}