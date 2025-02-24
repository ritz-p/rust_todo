use shared_struct::todo::mount::object::{
    create_todo::CreateTodo, todo::Todo, update_todo::UpdateTodo,
};
use tauri_plugin_http::reqwest::Client;
#[tauri::command]
pub async fn get(url: String) -> Result<Vec<Todo>, String> {
    let client = Client::new();
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
    let data: Vec<Todo> = response.json().await.map_err(|e| e.to_string())?;

    Ok(data)
}

#[tauri::command]
pub async fn post(url: String, body: CreateTodo) -> Result<(), String> {
    let client = Client::new();
    client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn patch(url: String, body: UpdateTodo) -> Result<(), String> {
    let client = Client::new();
    client
        .patch(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete(url: String) -> Result<(), String> {
    let client = Client::new();
    client.delete(url).send().await.map_err(|e| e.to_string())?;
    Ok(())
}
