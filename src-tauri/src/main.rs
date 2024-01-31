// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct SerializableResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
async fn cors_proxy (url: String, method: String, headers: Option<HashMap<String, String>>, body: Option<String>) -> Result<SerializableResponse, String> {
   let client = reqwest::Client::new();
   let request_builder = client.request(
    method.parse::<reqwest::Method>().map_err(|e| e.to_string())?,
    &url
   );
   let request_builder = if let Some(h) = headers {
    h.iter().fold(request_builder, |builder, (k,v)| builder.header(k, v))
   } else {
    request_builder
   };
   let request_builder = if let Some(b) = body {
    request_builder.body(b)
   } else {
    request_builder
   };
   let response = request_builder.send().await.map_err(|e| e.to_string())?;
   let status = response.status().as_u16();
   let headers  = response.headers().iter().map(|(k,v)| (k.to_string(), v.to_str().unwrap_or_default().to_string())).collect();
   let body = response.text().await.map_err(|e| e.to_string())?;
    Ok(SerializableResponse {
        status,
        headers,
        body,
    })
   
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, cors_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
