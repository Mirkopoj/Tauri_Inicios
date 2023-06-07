// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod buttons;
use buttons::button;

mod commands;
use commands::ls;

mod grafico;
use grafico::grafico;

mod usb;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, ls, button, grafico])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Holii, {}!", name)
}
