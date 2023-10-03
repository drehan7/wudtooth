// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use serde::{Serialize, Deserialize};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been doofed from Rust!", name)
}

type Devices = Vec<Device>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Device {
    name: String,
    mac_addr: String,
}

impl Device {
    fn new(device_info: &str) -> Device {
        // device_info comes in as a string.
        // Split by newline and then by space
        // Exs:
        //  - Device C8:L3:M1:0C:F3:S1 MX Master 2S
        //  - Device C8:L3:M1:0C:F3:S1 WudSound

        let tmp: Vec<String> = device_info.split(' ')
            .map(|s| s.to_string())
            .collect();

        if tmp.len() >= 3 {
            Device {
                name: String::from(tmp[2..].join(" ")),
                mac_addr: String::from(tmp[1].to_owned()),
            }
        }
        else {
            Device {
                name: String::from(""),
                mac_addr: String::from(""),
            }
        }
    }
}

// Return devices in Vec
// maybe make a struct
#[tauri::command]
fn get_devices() -> Vec<Device> {
    let call = Command::new("bluetoothctl")
        .arg("devices")
        .output()
        .unwrap()
        .stdout;

    // List of devices connected
    let devices: Devices = String::from_utf8(call)
        .unwrap()
        .split('\n')
        .map(|s| Device::new(s))
        .collect();

    println!("m_devices; {:?}", devices);

    return devices;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
