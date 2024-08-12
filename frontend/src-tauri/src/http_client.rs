use tauri::http::{Request, RequestBuilder, ResponseType};
use shared_struct::todo::mount::object::todo::Todo;
#[tauri::command]
async fn fetch(url: String) -> Result<Vec<Todo>,String> {
    let client = ClientBuilder::new().build().unwrap();

    let request = RequestBuilder::new("GET", &url)
        .response_type(ResponseType::Json)
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.send(request).await.map_err(|e| e.to_string())?;


    let data:Vec<Todo> = response
        .body()
        .map_err(|e| e.to_string())?;

    Ok(data)
}
