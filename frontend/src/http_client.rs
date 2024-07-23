use reqwest::Client;
use reqwest::Error;

struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    fn new(base_url: &str) -> Self {
        HttpClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    async fn get(&self, endpoint: &str) -> Result<String, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?.text().await?;
        Ok(response)
    }
}
