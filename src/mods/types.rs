use std::{error::Error, fmt};

use super::build_client::build_request;

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

pub struct JxufeClient {
    pub username: String,
    pub password: String,
    pub cookie: String,
    pub client: reqwest::Client,
}

impl JxufeClient {
    pub fn new(
        username: &str,
        password: &str,
        cookie: &str,
        proxy_url: &str,
        user_agent: &str,
    ) -> Result<Self, &'static str> {
        let client = match build_request(cookie, user_agent, proxy_url) {
            Ok(value) => value,
            Err(value) => {
                return Err(value);
            }
        };

        Ok(Self {
            username: username.to_owned(),
            password: password.to_owned(),
            cookie: cookie.to_owned(),
            client: client,
        })
    }

    pub fn set_cookie(&mut self, cookie: &str) {
        self.cookie = cookie.to_owned();
    }

    pub fn check_login_info(&self) -> Result<(), LoginError> {
        if self.username.len() == 0 {
            return Err(LoginError::InvalidUsername);
        }
        if self.password.len() == 0 {
            return Err(LoginError::InvalidPassword);
        }
        Ok(())
    }
}
