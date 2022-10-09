use reqwest::{header::HeaderMap, Client};

pub fn build_proxy_client() -> Client {
    let proxy_url = "http://127.0.0.1:10809";
    let mut headers = HeaderMap::new();
    headers.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .proxy(reqwest::Proxy::all(proxy_url).unwrap())
        .build()
        .unwrap();
    client
}

pub fn build_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    client
}
