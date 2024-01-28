use axum::http::{StatusCode};
use axum::response::{Response, IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // AUTH
    AuthFailNoAuthToken,
    AuthFailTokenWrongFormat,

    // MODEL ERRORS
    MessageIdNotFound{id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()

    }
}