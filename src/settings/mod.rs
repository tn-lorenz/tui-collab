use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    window_highlight: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_highlight: String::from("lightcyan"),
        }
    }
}

impl Settings {
    pub fn settings_path() -> PathBuf {
        let base = dirs::config_dir().expect("No config file found.");

        base.join("tui-collab").join("settings.json")
    }

    pub fn load_or_create() -> Self {
        let path = Self::settings_path();

        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&data) {
                return settings;
            }
        }

        let settings = Settings::default();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&path, serde_json::to_string_pretty(&settings).unwrap());
        settings
    }

    pub fn window_highlight_color(&self) -> Color {
        match self.window_highlight.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "lightcyan" => Color::LightCyan,
            "white" => Color::White,
            _ => Color::Reset,
        }
    }
}
