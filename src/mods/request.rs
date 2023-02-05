use reqwest::{header::HeaderMap, Client};
use std::{collections::HashMap, time::Duration};

pub async fn async_getwebpage(
    client: Option<Client>,
    url: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<(HashMap<String, String>, String, Option<Client>), ()> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(());
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(());
            }
        });
    }
    let use_exist_client = client.is_some();
    let client_for_return = {
        if use_exist_client {
            client
        } else {
            if let Ok(value) = client_builder
                .brotli(true)
                .gzip(true)
                .deflate(true)
                .timeout(Duration::from_secs(20))
                .user_agent(user_agent)
                .http1_title_case_headers()
                .build()
            {
                Some(value)
            } else {
                return Err(());
            }
        }  
    };
    let mut client = {
        if let Some(value) = client_for_return.clone() {
            value
        } else {
            return Err(());
        }
    }
    .get(url);
    if let Some(value) = headers {
        client = client
            .headers(value)
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }else{
        client = client
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(());
    };
    match rsp_raw_data.status().as_u16() {
        404 | 429 => return Err(()),
        _ => (),
    }
    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(());
    };
    Ok((rsp_headers, rsp_body, client_for_return))
}

pub async fn async_postwebpage(
    client: Option<Client>,
    url: &str,
    content: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<(HashMap<String, String>, String, Option<Client>), ()> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(());
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(());
            }
        });
    }
    let use_exist_client = client.is_some();
    let client_for_return = {
        if use_exist_client {
            client
        } else {
            if let Ok(value) = client_builder
                .brotli(true)
                .gzip(true)
                .deflate(true)
                .timeout(Duration::from_secs(20))
                .user_agent(user_agent)
                .http1_title_case_headers()
                .build()
            {
                Some(value)
            } else {
                return Err(());
            }
        }
    };

    let mut client = {
        if let Some(value) = client_for_return.clone() {
            value
        }else{
            return Err(());
        }
    }
    .post(url)
    .body(content.to_owned());
    if let Some(value) = headers {
        client = client.headers(value);
    }
    client = client
        .header("Cookie", cookie)
        .header("Accept-Encoding", "gzip, deflate, br");
    // .header("Content-Type", "application/x-www-form-urlencoded");
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(());
    };

    match rsp_raw_data.status().as_u16() {
        404 | 429 => return Err(()),
        _ => (),
    }
    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(());
    };
    Ok((rsp_headers, rsp_body, client_for_return))
}
