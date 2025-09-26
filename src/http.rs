//! HTTP client utilities and header management for Stych.fr API

use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

/// Default headers for Stych.fr API requests
static DEFAULT_HEADERS: Lazy<HeaderMap> = Lazy::new(|| {
    let mut headers = HeaderMap::new();

    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:138.0) Gecko/20100101 Firefox/138.0",
        ),
    );
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3"),
    );
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br, zstd"),
    );
    headers.insert(
        "X-Requested-With",
        HeaderValue::from_static("XMLHttpRequest"),
    );
    headers.insert("Origin", HeaderValue::from_static("https://www.stych.fr"));
    headers.insert("Sec-GPC", HeaderValue::from_static("1"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("Priority", HeaderValue::from_static("u=0"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert("TE", HeaderValue::from_static("trailers"));

    headers
});

/// Create a new HTTP client with default configuration
pub fn create_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .build()
}

/// Get default headers for Stych.fr API requests
pub fn get_default_headers() -> HeaderMap {
    DEFAULT_HEADERS.clone()
}
