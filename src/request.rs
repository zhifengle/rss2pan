use std::{
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::ArgMatches;
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, Method, RequestBuilder,
};
use serde_json::Value;

fn load_yiyiwu_cookies() -> Option<String> {
    if !Path::new(".cookies").exists() {
        return None;
    }
    let result = fs::read_to_string(".cookies");
    if let Ok(s) = result {
        return Some(s);
    }
    None
}

pub fn build_proxy_client() -> Client {
    let mut proxy_url = "http://127.0.0.1:10809".to_string();
    if env::var("ALL_PROXY").is_ok() {
        proxy_url = env::var("ALL_PROXY").unwrap();
    } else if env::var("HTTPS_PROXY").is_ok() {
        proxy_url = env::var("HTTPS_PROXY").unwrap();
    }
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";
    let client = reqwest::ClientBuilder::new()
        .user_agent(ua)
        .cookie_store(true)
        .proxy(reqwest::Proxy::all(proxy_url).unwrap())
        .build()
        .unwrap();
    client
}

pub fn build_client() -> Client {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";
    let client = reqwest::ClientBuilder::new()
        .user_agent(ua)
        .cookie_store(true)
        .build()
        .unwrap();
    client
}

pub struct Ajax {
    inner_client: reqwest::Client,
    inner_client_proxy: reqwest::Client,
    site_config: Value,
}

fn get_site_config(filename: Option<PathBuf>) -> serde_json::Value {
    let config: serde_json::Value = match filename {
        Some(p) => {
            let contents = std::fs::read_to_string(p).unwrap();
            serde_json::from_str(&contents).unwrap()
        }
        None => {
            let filename = "node-site-config.json";
            if Path::new(filename).exists() {
                let contents = std::fs::read_to_string(filename).unwrap();
                return serde_json::from_str(&contents).unwrap();
            } else {
                let filename = dirs::home_dir().unwrap().join(filename);
                if Path::new(&filename).exists() {
                    let contents = std::fs::read_to_string(filename).unwrap();
                    return serde_json::from_str(&contents).unwrap();
                }
                return serde_json::from_str("{}").unwrap();
            }
        }
    };
    config
}

impl Ajax {
    pub fn new() -> Self {
        Self {
            inner_client: build_client(),
            inner_client_proxy: build_proxy_client(),
            site_config: get_site_config(None),
        }
    }
    pub fn from_matches(_matches: &ArgMatches) -> Self {
        Self {
            inner_client: build_client(),
            inner_client_proxy: build_proxy_client(),
            site_config: get_site_config(None),
        }
    }
    fn get_cookie(&self, url: &str) -> Option<String> {
        let mut cookie = None;
        if url.contains("115.com") {
            cookie = load_yiyiwu_cookies();
        }
        cookie
    }
    pub fn gen_req(&self, method: Method, url: &str) -> RequestBuilder {
        let url_obj = url::Url::parse(url).unwrap();
        let host = url_obj.host_str().unwrap().to_string();
        let config = &self.site_config[host];
        let mut headers = HeaderMap::new();
        let headers_config = &config["headers"];
        if headers_config.is_object() {
            for (k, v) in headers_config.as_object().unwrap().iter() {
                headers.insert(
                    HeaderName::from_str(k.as_str()).unwrap(),
                    // HeaderValue::from_str(v.as_str().unwrap()).unwrap(),
                    v.as_str().unwrap().parse().unwrap(),
                );
            }
            // 使用的小写。
            if headers_config["cookie"].is_null() {
                if let Some(cookie) = self.get_cookie(url) {
                    headers.insert("Cookie", cookie.parse().unwrap());
                }
            }
        }
        if config["httpsAgent"].is_null() {
            return self.inner_client.request(method, url).headers(headers);
        } else {
            return self
                .inner_client_proxy
                .request(method, url)
                .headers(headers);
        }
    }
    pub fn gen_req_host(&self, method: Method, url: &str, host: &str) -> RequestBuilder {
        let config = &self.site_config[host];
        let mut headers = HeaderMap::new();
        let headers_config = &config["headers"];
        if headers_config.is_object() {
            for (k, v) in headers_config.as_object().unwrap().iter() {
                headers.insert(
                    HeaderName::from_str(k.as_str()).unwrap(),
                    v.as_str().unwrap().parse().unwrap(),
                );
            }
        }
        // 使用的小写。
        if config.is_null() || headers_config["cookie"].is_null() {
            if let Some(cookie) = self.get_cookie(host) {
                headers.insert("Cookie", cookie.parse().unwrap());
            }
        }
        if config["httpsAgent"].is_null() {
            return self.inner_client.request(method, url).headers(headers);
        } else {
            return self
                .inner_client_proxy
                .request(method, url)
                .headers(headers);
        }
    }
}
