#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use simconnect_sdk::{ Notification, SimConnect, SimConnectObject };
use tauri::InvokeError;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize, SimConnectObject)]
#[simconnect(period = "second")]
#[allow(dead_code)]
struct AircraftUserData {
    #[simconnect(name = "TITLE")]
    aircraft: String,
    #[simconnect(name = "CATEGORY")]
    category: String,
    #[simconnect(name = "AIRSPEED INDICATED", unit = "knots")]
    airspeed: f64,
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    lon: f64,
    #[simconnect(name = "PLANE ALTITUDE", unit = "feet")]
    alt: f64,
    #[simconnect(name = "AIRSPEED TRUE", unit = "knots")]
    true_airspeed: f64,
}

#[tauri::command]
fn start_simconnect_listener(window: tauri::Window) -> Result<(), InvokeError> {
    std::thread::spawn(move || {
        let mut client = match SimConnect::new("Receiving data from Simconnect") {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to connect to SimConnect: {:?}", e);
                return;
            }
        };

        client.register_object::<AircraftUserData>().expect("Failed to register object");

        loop {
            match client.get_next_dispatch() {
                Ok(Some(Notification::Object(data))) => {
                    if let Ok(airplane_data) = AircraftUserData::try_from(&data) {
                        // println!("Airplane Data: {:?}", airplane_data); 
                        window
                            .emit("simconnect-data", airplane_data)
                            .expect("Failed to emit data event");
                    }
                }
                _ => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    });

    Ok(())
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![my_custom_command, start_simconnect_listener])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
