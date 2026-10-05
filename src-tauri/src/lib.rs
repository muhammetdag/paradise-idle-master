mod steam_api;

use steam_api::{start_idler as start_idler_internal, stop_idler as stop_idler_internal};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SavedGame {
    app_id: String,
    name: String,
    image: Option<String>,
    elapsed_time: u64,
}

fn get_data_dir() -> Result<PathBuf, String> {
    let app_data = std::env::var("APPDATA")
        .map_err(|_| "Could not find AppData directory".to_string())?;
    let mut path = PathBuf::from(app_data);
    path.push("paradise_idle_master");
    
    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    Ok(path)
}

#[tauri::command]
fn save_favorites(games: Vec<SavedGame>) -> Result<(), String> {
    let mut path = get_data_dir()?;
    path.push("favorites.bin");
    
    let json = serde_json::to_string(&games)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    
    fs::write(&path, json)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn load_favorites() -> Result<Vec<SavedGame>, String> {
    let mut path = get_data_dir()?;
    path.push("favorites.bin");
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let games: Vec<SavedGame> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to deserialize: {}", e))?;
    
    Ok(games)
}

#[tauri::command]
fn save_language(lang: String) -> Result<(), String> {
    let mut path = get_data_dir()?;
    path.push("language.txt");
    
    fs::write(&path, lang)
        .map_err(|e| format!("Failed to write language: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn load_language() -> Result<String, String> {
    let mut path = get_data_dir()?;
    path.push("language.txt");
    
    if !path.exists() {
        return Ok("en".to_string());
    }
    
    let lang = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read language: {}", e))?;
    
    Ok(lang)
}

#[derive(Debug, Serialize, Deserialize)]
struct SteamSearchItem {
    id: u32,
    name: String,
    tiny_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SteamSearchResponse {
    total: u32,
    items: Vec<SteamSearchItem>,
}

#[tauri::command]
async fn search_steam_games(query: String) -> Result<SteamSearchResponse, String> {
    let url = format!(
        "https://store.steampowered.com/api/storesearch/?term={}&l=turkish&cc=tr",
        urlencoding::encode(&query)
    );
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let data: SteamSearchResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(data)
}

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct SteamAppData {
    name: String,
    header_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SteamAppDetails {
    success: bool,
    data: Option<SteamAppData>,
}

#[tauri::command]
async fn get_steam_game_details(app_id: String) -> Result<SteamAppData, String> {
    if app_id.is_empty() || !app_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("Invalid App ID. App ID must contain only digits.".to_string());
    }

    let url = format!(
        "https://store.steampowered.com/api/appdetails?appids={}&l=turkish",
        app_id
    );
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let mut data: HashMap<String, SteamAppDetails> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    let details = data.remove(&app_id).ok_or("No data found for this AppID")?;
    
    if details.success {
        details.data.ok_or_else(|| "Game data not found".to_string())
    } else {
        Err("Game not found on Steam".to_string())
    }
}

#[tauri::command]
fn start_idler(app_id: String) -> Result<(), String> {
    start_idler_internal(app_id)
}

#[tauri::command]
fn stop_idler(app_id: String) -> Result<(), String> {
    stop_idler_internal(&app_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "--idle-worker" {
        steam_api::run_headless_idler(args[2].clone());
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let quit_i = tauri::menu::MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&quit_i])?;

            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    match event {
                        tauri::tray::TrayIconEvent::Click { 
                            button: tauri::tray::MouseButton::Left,
                            ..
                        } => {
                            if let Some(window) = tray.app_handle().get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            start_idler, 
            stop_idler, 
            search_steam_games,
            get_steam_game_details,
            save_favorites,
            load_favorites,
            save_language,
            load_language
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
