// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn restart_firefox() {
    if cfg!(windows) {
        println!("this is windows");
        Command::new("taskkill")
            .args(["/F", "/T", "/IM", "firefox.exe"])
            .spawn()
            .expect("couldnt kill firefox!");

        Command::new("cmd")
            .args(["/C", "start firefox.exe"])
            .spawn()
            .expect("couldnt spawn firefox");
    } else if cfg!(unix) {
        println!("this is unix");
        Command::new("killall")
            .arg("firefox")
            .spawn()
            .expect("couldnt kill firefox!");
        Command::new("firefox")
            .spawn()
            .expect("couldnt spawn firefox");
    } else {
        panic!("something went wrong")
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![restart_firefox])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
