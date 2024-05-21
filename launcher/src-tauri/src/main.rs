#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![windows_subsystem = "windows"]

use io::Result;
use std::{io, process::Command, thread::sleep, time::Duration};

fn run_command_in_console(command: &str, args: &[&str]) -> io::Result<()> {
    // Create a new command for cmd.exe
    let mut cmd = Command::new("cmd");

    // Add the /K flag to keep the console open and the command to execute
    cmd.arg("/K").arg(command).args(args);

    // Spawn the command and wait for it to complete
    let status = cmd.spawn()?;



    Ok(())
}

#[tauri::command]
fn runfile(name: &str) -> String {
    run_command_in_console("E:/Dev/priede/releases/0-1-0/priede_cli_0-1-0.exe", &[name]);
    /* 
    let mut command = Command::new("E:/Dev/priede/releases/0-1-0/priede_cli_0-1-0.exe");
    command.arg(name);
    let mut handle = command.spawn();
    sleep(Duration::from_secs(5));
    handle.unwrap().wait();
    */
    return "a".to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![runfile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
