// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::str;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, ls])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Holii, {}!", name)
}

#[tauri::command]
fn ls(args: [&str;2]) -> String {
    str::from_utf8(
        &Command::new("ls")
                .args(&args)
                .output()
                .expect("ou nou fallou")
                .stdout
            )
        .expect("Fallo from_utf8")
        .to_string()
        .to_html()
}

trait ToHTML{
    fn to_html(&self) -> String;
}

impl ToHTML for String {
    fn to_html(&self) -> String {
        self.replace("\n", "<br>")
    }
}