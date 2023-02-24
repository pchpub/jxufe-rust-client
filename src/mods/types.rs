use std::{collections::HashMap, error::Error, fmt};

use reqwest::header::HeaderMap;

use super::{build_client::build_request, getinfo::login::login, request::{async_getwebpage, async_postwebpage}};

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
    pub client: reqwest::Client,
    user_agent: String,
    proxy_url: String,
    cookie_map: HashMap<String,String>,
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
            client,
            user_agent: user_agent.to_owned(),
            proxy_url: proxy_url.to_owned(),
            cookie_map: Self::cookie_str_to_map(cookie),
        })
    }

    pub fn set_cookie(&mut self, cookie: &str) {
        self.cookie_map = Self::cookie_str_to_map(cookie);
        self.client = match build_request(cookie, &self.user_agent, &self.proxy_url) {
            Ok(value) => value,
            Err(_) => {
                panic!("set cookie failed");
            }
        };
    }

    pub fn cookie(&self) -> String {
        Self::cookie_map_to_str(&self.cookie_map)
    }

    pub fn set_cookie_map(&mut self, cookie: &str) {
        self.cookie_map = Self::cookie_str_to_map(cookie);
    }

    fn cookie_str_to_map(cookie: &str) -> HashMap<String, String> {
        let mut cookie_map = HashMap::new();
        let cookie_list = cookie.split(";");
        for item in cookie_list {
            let cookie_item = item.trim().split("=").collect::<Vec<&str>>();
            if cookie_item.len() != 2 {
                continue;
            }
            cookie_map.insert(cookie_item[0].to_owned(), cookie_item[1].to_owned());
        }
        cookie_map
    }

    fn cookie_map_to_str(cookie_map: &HashMap<String, String>) -> String {
        let mut cookie_str = String::new();
        for (key, value) in cookie_map {
            cookie_str.push_str(&format!("{}={};", key, value));
        }
        cookie_str
    }

    fn cookie_map_add(&mut self, key: &str, value: &str) {
        self.cookie_map.insert(key.to_owned(), value.to_owned());
    }

    // fn cookie_map_remove(&mut self, key: &str) {
    //     self.cookie_map.remove(key);
    // }

    fn cookie_map_add_string(&mut self, cookie: &str) {
        let cookie_list = cookie.split(";");
        for item in cookie_list {
            let cookie_item = item.trim().split("=").collect::<Vec<&str>>();
            if cookie_item.len() != 2 {
                continue;
            }
            self.cookie_map_add(cookie_item[0], cookie_item[1]);
        }
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

    pub async fn login(&mut self) -> Result<(), LoginError> {
        login(self).await
    }

    pub async fn async_getwebpage(
        &mut self,
        url: &str,
        headers: Option<HeaderMap>,
    ) -> Result<(HashMap<String, String>, String), ()> {
        let raw_data = async_getwebpage(&mut self.client, url, headers).await;
        match &raw_data {
            Ok(value) => {
                let (headers, _body) = value;
                if headers.contains_key("set-cookie") {
                    let cookie = headers.get("set-cookie").unwrap().as_str();
                    self.cookie_map_add_string(cookie);
                }
            }
            Err(_) => (),
        }
        raw_data
    }

    pub async fn async_postwebpage(
        &mut self,
        url: &str,
        headers: Option<HeaderMap>,
        content: &str,
    ) -> Result<(HashMap<String, String>, String), ()> {
        let raw_data = async_postwebpage(&mut self.client, url, content, headers).await;
        match &raw_data {
            Ok(value) => {
                let (headers, _body) = value;
                if headers.contains_key("set-cookie") {
                    let cookie = headers.get("set-cookie").unwrap().as_str();
                    self.cookie_map_add_string(cookie);
                }
            }
            Err(_) => (),
        }
        raw_data
    }
}
