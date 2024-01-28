use axum::http::{StatusCode};
use axum::response::{Response, IntoResponse};
use serde::Serialize;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, AsRefStr)]
#[serde(tag = "type", content = "why")]
pub enum Error {
    LoginFail,

    // AUTH
    AuthFailNoAuthToken,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // Internal stuff
    DatabaseConnectionError,
    DatabaseQueryError,
    DatabaseParseError,

    // MODEL ERRORS
    MessageIdNotFound{id: u64},
}

#[derive(Debug, AsRefStr, Serialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type", content = "why")]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    BAD_REQUEST {why: String},
    INTERNAL_ERROR {why: String}
}

impl Error { 
    pub fn to_status_and_client_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::AuthFailTokenWrongFormat |
            Self::AuthFailNoAuthToken |
            Self::AuthFailCtxNotInRequestExt => (
                StatusCode::FORBIDDEN,
                ClientError::LOGIN_FAIL
            ),
            Self::MessageIdNotFound { id } => (
                StatusCode::BAD_REQUEST,
                ClientError::BAD_REQUEST { why: format!("Could not find message with id {id}") }
            ),
            Self::DatabaseConnectionError | 
            Self::DatabaseParseError | 
            Self::DatabaseQueryError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_ERROR { why: "Internal database error.".to_string() }
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_ERROR { why: "Something went wrong...".to_string() }
            )
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // placeholder response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}
