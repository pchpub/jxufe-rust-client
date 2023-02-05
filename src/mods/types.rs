use std::{error::Error, fmt};

#[derive(Debug)]
pub enum LoginError {
    InvalidUsername,
    InvalidPassword,
    UserNotFound,
    PasswordMismatch,
    NetworkError,
    OtherError,
}

impl Error for LoginError {
    fn description(&self) -> &str {
        match *self {
            LoginError::InvalidUsername => "invalid username",
            LoginError::InvalidPassword => "invalid password",
            LoginError::UserNotFound => "user not found",
            LoginError::PasswordMismatch => "password mismatch",
            LoginError::NetworkError => "network error",
            LoginError::OtherError => "other error",
        }
    }
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoginError::InvalidUsername => write!(f, "invalid username"),
            LoginError::InvalidPassword => write!(f, "invalid password"),
            LoginError::UserNotFound => write!(f, "user not found"),
            LoginError::PasswordMismatch => write!(f, "password mismatch"),
            LoginError::NetworkError => write!(f, "network error"),
            LoginError::OtherError => write!(f, "other error"),
        }
    }
}

impl From<()> for LoginError {
    fn from(_: ()) -> Self {
        LoginError::OtherError
    }
}