use reqwest::Client;
use shared_struct::todo::mount::object::todo::Todo;
#[tauri::command]
pub async fn fetch(url: String) -> Result<Vec<Todo>, String> {
    let client = Client::new();
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
    let data: Vec<Todo> = response.json().await.map_err(|e| e.to_string())?;

    Ok(data)
}
