use std::process::Command;
use std::str;

trait ToHTML{
    fn to_html(&self) -> String;
}

impl ToHTML for String {
    fn to_html(&self) -> String {
        self.replace("\n", "<br>")
    }
}

#[tauri::command]
pub fn ls(args: [&str;2]) -> String {
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

