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
pub fn start_simconnect_listener(window: tauri::Window) -> Result<(), InvokeError> {
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