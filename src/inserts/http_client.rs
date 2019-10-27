use reqwest::{Client, IntoUrl, RequestBuilder};
use serenity::prelude::TypeMapKey;
use std::sync::{Arc, Mutex};

// TODO: Use one reqwest::Client

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    #[inline]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    #[inline]
    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.client.post(url)
    }
}

pub struct HttpClientKey;

impl TypeMapKey for HttpClientKey {
    type Value = Arc<Mutex<HttpClient>>;
}
