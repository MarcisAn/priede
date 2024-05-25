#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![windows_subsystem = "windows"]

use io::Result;
use std::{io, process::Command, thread::sleep, time::Duration};

fn run_command_in_console(command: &str, args: &[&str])  {
    let mut cmd = Command::new("cmd");

    cmd.arg("/K").arg(command).args(args);

    let status = cmd.spawn();
}

#[tauri::command]
fn runfile(name: &str) {
    run_command_in_console("priede.exe", &[name, "v0", "halt"]);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![runfile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
