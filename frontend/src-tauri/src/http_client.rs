use tauri::http::Request;
pub struct HttpClient {
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            base_url: base_url.to_string(),
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<String, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = Request::get(&url).await?.text().await?;
        Ok(response)
    }
}
