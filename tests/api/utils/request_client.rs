use reqwest::{Client, Error, Response};
use serde::Serialize;

pub struct RequestClient {
    client: Client,
    base_url: String,
}

impl RequestClient {
    pub fn new(base_url: &str) -> RequestClient {
        RequestClient {
            client: Client::new(),
            base_url: base_url.to_owned(),
        }
    }

    pub async fn get(&self, path: &str) -> Result<Response, Error> {
        let base_url = self.base_url.as_str();
        self.client.get(format!("{base_url}{path}")).send().await
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        data: &T,
    ) -> Result<Response, Error> {
        let base_url = self.base_url.as_str();
        self.client
            .post(format!("{base_url}{path}"))
            .json(&data)
            .send()
            .await
    }
}
