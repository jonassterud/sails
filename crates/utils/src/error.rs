use std::fmt::{Debug, Display};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Error.
#[derive(Clone)]
pub struct Error {
    error: String,
    status: Option<StatusCode>,
}

impl Error {
    /// Create `Error` with any error and `None` as `StatusCode`.
    pub fn new<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self {
            error: error.to_string(),
            status: None,
        }
    }

    /// Add a status.
    pub fn with(mut self, status: StatusCode) -> Self {
        self.status = Some(status);

        return self;
    }

    /// Create `Error` with status `500 Internal Server Error`.
    #[allow(non_snake_case)]
    pub fn INTERNAL_SERVER_ERROR<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error).with(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Create `Error` with status `400 Bad Request`.
    #[allow(non_snake_case)]
    pub fn BAD_REQUEST<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error).with(StatusCode::BAD_REQUEST)
    }

    /// Create `Error` with status `401 Unauthorized`.
    #[allow(non_snake_case)]
    pub fn UNAUTHORIZED<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error).with(StatusCode::UNAUTHORIZED)
    }

    /// Create `Error` with status `403 Forbidden`.
    #[allow(non_snake_case)]
    pub fn FORBIDDEN<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error).with(StatusCode::FORBIDDEN)
    }

    /// Create `Error` with status `403 Forbidden`.
    #[allow(non_snake_case)]
    pub fn NOT_FOUND<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error).with(StatusCode::NOT_FOUND)
    }

    /// Create `Error` with status `409 Conflict`.
    #[allow(non_snake_case)]
    pub fn CONFLICT<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error).with(StatusCode::CONFLICT)
    }

    /// Create `Error` with status `511 Network Authentication Required`.
    #[allow(non_snake_case)]
    pub fn NETWORK_AUTHENTICATION_REQUIRED<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error).with(StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            format!("Something went wrong: {}", self.error),
        )
            .into_response()
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        Self::new(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Some(status) => write!(f, "{}: {}", status, self.error),
            None => write!(f, "{}", self.error),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Some(status) => write!(f, "{}: {}", status, self.error),
            None => write!(f, "{}", self.error),
        }
    }
}
