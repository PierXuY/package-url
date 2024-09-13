use serde::{Deserialize, Serialize};
use std::fs;
use tauri_utils::config::WindowConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub url_window: WindowConfig,
    pub windows_cmd: Vec<String>,
    pub emit_message: bool,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum WindowsCmd {
//     Single(String),
//     Multiple(Vec<String>),
// }

// impl WindowsCmd {
//     pub fn as_str(&self) -> String {
//         match self {
//             WindowsCmd::Single(s) => s.clone(),
//             WindowsCmd::Multiple(v) => v.join("&"),
//         }
//     }

//     pub fn is_non_empty(&self) -> bool {
//         match self {
//             WindowsCmd::Single(s) => !s.is_empty(),
//             WindowsCmd::Multiple(v) => !v.is_empty(),
//         }
//     }
// }

pub fn parse_config(config_path: std::path::PathBuf) -> Config {
    let conf_json_path = config_path.join("config.json");
    let file_content = fs::read_to_string(conf_json_path).unwrap();
    let config: Config = serde_json::from_str(&file_content).unwrap();
    config
}
