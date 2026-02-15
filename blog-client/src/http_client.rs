use reqwest::Url;

pub struct HttpClient {
    base_url: Url,
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new(base_url: Url, client: reqwest::Client) -> Self {
        Self {base_url, client: reqwest::Client::new() }
    }



}