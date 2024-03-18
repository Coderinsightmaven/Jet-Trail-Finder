#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri;
pub mod simconnect_listener; // Make the module public

fn main() {
    tauri::Builder::default()
        // Import the command handler into the current scope
        .invoke_handler(tauri::generate_handler![simconnect_listener::start_simconnect_listener])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
