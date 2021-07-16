//! Errors that can occur during middleware processing stage
use actix_web::{ResponseError, HttpResponse};
use failure::{self, Fail};
use std::time::Duration;

/// Custom error type. Useful for logging and debugging different kinds of errors.
/// This type can be converted to Actix Error, which defaults to
/// InternalServerError
///
#[derive(Debug, Fail)]
pub enum ARError {
    /// Store is not connected
    #[fail(display = "store not connected")]
    NotConnected,

    /// Store is disconnected after initial successful connection
    #[fail(display = "store disconnected")]
    Disconnected,

    /// Read/Write error on store
    #[fail(display = "read/write operatiion failed: {}", _0)]
    ReadWriteError(String),

    /// Could be any kind of IO error
    #[fail(display = "unknown error: {}", _0)]
    UnknownError(std::io::Error),

    /// Identifier error
    #[fail(display = "client identification failed")]
    IdentificationError,

    #[fail(display = "too many requests")]
    TooManyRequests(usize, usize, Duration),
}

impl ResponseError for ARError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ARError::TooManyRequests(limit, remaining, reset) =>
                HttpResponse::TooManyRequests()
                  //.insert_header(("x-ratelimit-limit", limit.to_string()))
                  //.insert_header(("x-ratelimit-remaining", remaining.to_string()))
                  //.insert_header(("x-ratelimit-reset", reset.as_secs().to_string()))
                  .finish(),
            _ =>
                HttpResponse::InternalServerError().finish()
        }
    }
}
