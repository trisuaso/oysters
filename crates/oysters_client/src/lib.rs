use oysters_core::pearl::ResourceDescriptor;
use reqwest::{self, StatusCode};

#[derive(Clone)]
pub struct Client {
    pub url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{path}", self.url)
    }

    pub async fn dump(&self) {
        let req = self.client.post(self.build_url("_dump"));
        req.send().await.unwrap();
    }

    pub async fn scan(&self) {
        let req = self.client.post(self.build_url("_scan"));
        req.send().await.unwrap();
    }

    pub async fn get(&self, key: &str) -> String {
        let req = self.client.get(self.build_url(key));
        req.send().await.unwrap().text().await.unwrap()
    }

    pub async fn insert(&self, key: &str, value: &str) -> bool {
        let req = self
            .client
            .post(self.build_url(key))
            .body(value.to_string());
        req.send().await.unwrap().status() == StatusCode::OK
    }

    pub async fn incr(&self, key: &str) -> bool {
        let req = self.client.post(self.build_url(&format!("_incr/{key}")));
        req.send().await.unwrap().status() == StatusCode::OK
    }

    pub async fn decr(&self, key: &str) -> bool {
        let req = self.client.post(self.build_url(&format!("_decr/{key}")));
        req.send().await.unwrap().status() == StatusCode::OK
    }

    pub async fn remove(&self, key: &str) -> String {
        let req = self.client.delete(self.build_url(key));
        req.send().await.unwrap().text().await.unwrap()
    }

    pub async fn filter(&self, pattern: &str) -> Vec<(String, (String, ResourceDescriptor))> {
        let req = self
            .client
            .post(self.build_url("_filter"))
            .body(pattern.to_string());
        req.send().await.unwrap().json().await.unwrap()
    }

    pub async fn filter_keys(&self, pattern: &str) -> Vec<String> {
        let req = self
            .client
            .post(self.build_url("_filter/keys"))
            .body(pattern.to_string());
        req.send().await.unwrap().json().await.unwrap()
    }
}
