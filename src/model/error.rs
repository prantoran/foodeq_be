use axum::{http::Response, response::IntoResponse};
use reqwest::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr}; // for serializing errors that do not implement Serialize (i.e. sqlx::Error)
use tracing::info;

use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as] // serde_as has to be before Serialize
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)] // TODO: remove this when all errors are handled
pub enum Error {
    // -- Modules
    // -- Store errors
    Store(String), // implementing variant, we can see what type of module error it is by matching the enum

    // -- Config
    ConfigMissingEnv(&'static str),

    // -- Login errors
    LoginFail,

    // -- Model errors
    TicketDeleteFailIdNotFound {
        id: u64,
    },

    // -- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // -- External Service errors
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error), // We do not have Serialize for sqlx::Error, so we cannot derive Serialize for Error if we embed sqlx::Error directly.
}

// region: --Froms

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}

// Going from DB store::Error to model::Error
impl From<store::error::Error> for Error {
    fn from(e: store::Error) -> Self {
        Error::Store(e.to_string())
    }
}

// endregion: --Froms

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response<axum::body::Body> {
        info!("{:<12} - {self:?}", "INTO_RES");
        
        // Create a placeholder Axum response
        let response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the error into the response
        // response.extensions_mut().insert(self); // requires Clone trait on Error

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // - Auth errors
            Self::AuthFailNoAuthTokenCookie
                                    | Self::AuthFailTokenWrongFormat
                                    | Self::AuthFailCtxNotInRequestExt => {
                        (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
                    }
            // - Model errors
            Self::TicketDeleteFailIdNotFound { .. } => {
                        (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
                    }
            Error::ConfigMissingEnv(_) => todo!(),
            Error::Store(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
            Error::Sqlx(_) => (
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