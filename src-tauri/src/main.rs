#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::exit;

#[tauri::command]
fn quit_app() {
  exit(0)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![quit_app])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
