// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dunce::canonicalize;
use std::io;
use std::path::PathBuf;
use std::{thread, time};
use tauri::Window;
use tauri_utils::config::WindowConfig;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let config_path = resolve_config_path(app);

            let splash_window = create_splash_window(app);
            let url_window = create_url_window(app, config_path);

            splash_window.show().unwrap();
            monitor_url(url_window, splash_window);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn resolve_config_path(app: &tauri::App) -> PathBuf {
    let config_path = app
        .path_resolver()
        .resolve_resource("config")
        .expect("failed to resolve resource");

    canonicalize(config_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .unwrap()
}

fn create_splash_window(app: &tauri::App) -> Window {
    let handle = app.handle();
    let splash_window = tauri::WindowBuilder::new(
        &handle,
        "splash",
        tauri::WindowUrl::App("splash.html".into()),
    )
    .center()
    .decorations(false)
    .inner_size(200.0, 200.0)
    .transparent(true)
    .build()
    .unwrap();
    splash_window
}

fn parse_config(
    config_path: std::path::PathBuf,
) -> Result<WindowConfig, Box<dyn std::error::Error>> {
    let conf_json_path = config_path.join("conf.json");
    let contents = std::fs::read_to_string(conf_json_path)?;
    let mut json_data: serde_json::Value = serde_json::from_str(&contents)?;
    
    if let Some(obj) = json_data
        .get_mut("url_window")
        .and_then(serde_json::Value::as_object_mut)
    {
        obj.insert("label".to_string(), serde_json::json!("url_window"));
    }

    serde_json::from_value(json_data["url_window"].clone())
        .map_err(|_e| From::from("Failed to parse JSON data"))
}

fn create_url_window(app: &tauri::App, config_path: std::path::PathBuf) -> Window {
    let handle = app.handle();
    let url_window_config = parse_config(config_path).unwrap();
    let url_window = tauri::WindowBuilder::from_config(&handle, url_window_config)
        .visible(false)
        .initialization_script(include_str!("setupLinKS.js"))
        .build()
        .unwrap();

    url_window
}

fn monitor_url(url_window: Window, splash_window: Window) {
    tauri::async_runtime::spawn(async move {
        loop {
            let current_url = &url_window.url();
            let url_str = current_url.as_str();
            if url_str != "about:blank" {
                splash_window.close().unwrap();
                url_window.show().unwrap();
                break;
            }
            thread::sleep(time::Duration::from_secs_f32(0.3));
        }
    });
}
