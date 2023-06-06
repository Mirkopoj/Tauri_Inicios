// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, ls, button, button1])
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

#[tauri::command]
fn button() -> String {
    format!("Apretaste el botón!!")}

#[tauri::command]
fn button1() -> String {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    s
}