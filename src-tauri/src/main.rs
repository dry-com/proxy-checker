// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::blocking::Client;
use reqwest::Proxy;
use std::time::Instant;
use tauri::command;

#[derive(serde::Serialize)]
struct ConnectionResult {
    connection: bool,
    delay: u128,
}

#[command]
fn check_connection(
    proxy_url: &str,
    website_url: Option<&str>,
) -> Result<ConnectionResult, String> {
    // Use the provided website URL or default to example.com
    let url_to_check = website_url.unwrap_or("https://www.example.com");

    // Set up the proxy using the provided proxy URL
    let proxy = Proxy::all(proxy_url).map_err(|e| format!("Failed to set proxy: {}", e))?;

    // Create a client with the proxy
    let client_with_proxy = Client::builder()
        .proxy(proxy)
        .build()
        .map_err(|e| format!("Failed to build client with proxy: {}", e))?;

    // Check connection with proxy
    let start = Instant::now();
    let response_with_proxy = client_with_proxy.get(url_to_check).send();
    let duration_with_proxy = start.elapsed().as_millis();

    // Determine if the connection was successful
    match response_with_proxy {
        Ok(_) => Ok(ConnectionResult {
            connection: true,
            delay: duration_with_proxy,
        }),
        Err(_) => Ok(ConnectionResult {
            connection: false,
            delay: 0,
        }),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
