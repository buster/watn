use std::fmt;

#[derive(Debug)]
pub enum Error {
    ConfigError(String),
    ProviderNotFound(String),
    AuthError(String),
    ApiError { status: u16, message: String },
    NetworkError(String),
    IoError(std::io::Error),
    Interrupted,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConfigError(msg) => write!(f, "config error: {}", msg),
            Error::ProviderNotFound(name) => write!(f, "unknown provider: {}", name),
            Error::AuthError(msg) => write!(f, "authentication error: {}", msg),
            Error::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            Error::NetworkError(msg) => write!(f, "network error: {}", msg),
            Error::IoError(e) => write!(f, "I/O error: {}", e),
            Error::Interrupted => write!(f, "interrupted"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

pub fn exit_code(err: &Error) -> i32 {
    match err {
        Error::ConfigError(_) | Error::ProviderNotFound(_) => 1,
        Error::AuthError(_) | Error::ApiError { .. } => 2,
        Error::NetworkError(_) => 3,
        Error::IoError(_) => 1,
        Error::Interrupted => 130,
    }
}
