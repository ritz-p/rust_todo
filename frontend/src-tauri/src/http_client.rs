use reqwest::{header::CONTENT_TYPE, Client};
use shared_struct::todo::mount::object::{create_todo::CreateTodo, todo::Todo};
#[tauri::command]
pub async fn fetch(url: String) -> Result<Vec<Todo>, String> {
    let client = Client::new();
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
    let data: Vec<Todo> = response.json().await.map_err(|e| e.to_string())?;

    Ok(data)
}

#[tauri::command]
pub async fn post(url: String, todo: CreateTodo) -> Result<(), String> {
    let client = Client::new();
    client
        .post(url)
        .json(&todo)
        .header(CONTENT_TYPE, "application_json")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete() -> Result<(), String> {
    todo!();
}
