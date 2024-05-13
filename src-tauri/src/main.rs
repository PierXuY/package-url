// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dunce::canonicalize;
use serde_json::{json, Value};
use std::fs::File;
use std::io;
use std::io::Read;
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

fn create_url_window(app: &tauri::App, config_path: std::path::PathBuf) -> Window {
    let conf_json_path = config_path.join("conf.json");

    let mut file = File::open(conf_json_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let handle = app.handle();
    let url_window_config: WindowConfig =
        serde_json::from_value(json_data.get("windows").unwrap().get(0).unwrap().clone())
            .expect("error");
    let url_window = tauri::WindowBuilder::from_config(&handle, url_window_config.clone())
        .visible(false)
        .build()
        .unwrap();

    url_window
}

fn monitor_url(url_window: Window, splash_window: Window) {
    tauri::async_runtime::spawn(async move {
        loop {
            let current_url = &url_window.url();
            let url_str = current_url.as_str();
            // 检查URL是否为 "about:blank"
            if url_str != "about:blank" {
                splash_window.close().unwrap();
                url_window.show().unwrap();
                break;
            }
            thread::sleep(time::Duration::from_secs_f32(0.3));
        }
    });
}
