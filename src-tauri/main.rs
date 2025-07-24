#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod czkfunc;
use czkfunc::CzkFUNC;

#[tauri::command]
fn execute(script: String) {
    CzkFUNC::execute(&script);
}

#[tauri::command]
fn inject() {
    CzkFUNC::inject();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute, inject])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o Tauri");
}
