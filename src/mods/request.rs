use reqwest::{header::HeaderMap, Client};
use std::{collections::HashMap};

pub async fn async_getwebpage(
    raw_client: &mut Client,
    url: &str,
    headers: Option<HeaderMap>,
) -> Result<(HashMap<String, String>, String), ()> {
    let mut client = raw_client.get(url);
    if let Some(value) = headers {
        client = client.headers(value)
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
    Ok((rsp_headers, rsp_body))
}

pub async fn async_postwebpage(
    client: &mut Client,
    url: &str,
    content: &str,
    headers: Option<HeaderMap>,
) -> Result<(HashMap<String, String>, String), ()> {
    let mut client = client.post(url).body(content.to_owned());
    if let Some(value) = headers {
        client = client.headers(value);
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
    Ok((rsp_headers, rsp_body))
}
