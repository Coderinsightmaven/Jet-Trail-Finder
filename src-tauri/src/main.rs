#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri;
pub mod simconnect_listener; // Make the module public
pub mod connect;

fn main() {
    // Run the database connection test
    if let Err(e) = tauri::async_runtime::block_on(connect::establish_connection()) {
        eprintln!("Failed to connect to database: {:?}", e);
    }

    tauri::Builder::default()
        // Import the command handler into the current scope
        .invoke_handler(tauri::generate_handler![simconnect_listener::start_simconnect_listener])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}