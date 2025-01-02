// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parse;

use std::io;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use command_group::{AsyncCommandGroup, AsyncGroupChild};
use dunce::canonicalize;
use parse::parse_config;
use tauri::{Manager, Window};
use tauri_utils::config::WindowConfig;
use winapi::um::winbase::CREATE_NO_WINDOW;

fn main() {
    let executed_windows_cmd_child = Arc::new(Mutex::new(None));
    tauri::Builder::default()
        // Ensure a single instance of your tauri app is running.
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .setup({
            let executed_windows_cmd_child = Arc::clone(&executed_windows_cmd_child);
            move |app| {
                let config_path = resolve_config_path(app);
                let mut config_json = parse_config(config_path);

                if !config_json.windows_cmd.is_empty() {
                    // let windows_command = config_json.windows_cmd.as_str();
                    *executed_windows_cmd_child.lock().unwrap() =
                        Some(execute_windows_cmds(config_json.windows_cmd));
                }

                let splash_window = create_splash_window(app);
                splash_window.show().unwrap();
                config_json.url_window.label = "url_window".to_string();
                let url_window = create_url_window(app, config_json.url_window);

                if config_json.emit_message {
                    app.listen_global("ready", move |_event| {
                        splash_window.close().unwrap();
                        url_window.show().unwrap();
                    });
                } else {
                    monitor_url(url_window, splash_window);
                }

                Ok(())
            }
        })
        .build(tauri::generate_context!())
        .unwrap()
        .run({
            let executed_windows_cmd_child = Arc::clone(&executed_windows_cmd_child);
            move |_app_handle, event| match event {
                tauri::RunEvent::ExitRequested { .. } => {
                    if let Some(windows_cmd_child) =
                        executed_windows_cmd_child.lock().unwrap().take()
                    {
                        kill_windows_cmd_children(windows_cmd_child);
                    }
                }
                _ => {}
            }
        })
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

fn create_url_window(app: &tauri::App, url_window_config: WindowConfig) -> Window {
    let handle = app.handle();
    let url_window = tauri::WindowBuilder::from_config(&handle, url_window_config)
        .visible(false)
        .initialization_script(include_str!("../scripts/setupLinKS.js"))
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

#[tokio::main(flavor = "current_thread")]
async fn execute_windows_cmds(commands: Vec<String>) -> Vec<AsyncGroupChild> {
    let mut children = Vec::new();

    for command in commands {
        let group = tokio::process::Command::new("cmd")
            .args(&["/C", command.as_str()])
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .group()
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .expect("Failed to start backend.");
        children.push(group);
    }

    children
}

#[tokio::main(flavor = "current_thread")]
async fn kill_windows_cmd_children(children: Vec<AsyncGroupChild>) {
    for mut child in children {
        child.kill().await.expect("command wasn't running");
    }
}
