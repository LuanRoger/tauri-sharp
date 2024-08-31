use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use tauri::App;

//TODO: Add support to pass the server executable name as an argument
fn get_server_path(file_name: Option<&str>) -> Option<PathBuf> {
    let server_exec = file_name.unwrap_or("TauriSharp.exe");
    let exec_dir = env::current_dir().unwrap();
    let server_path = exec_dir.join(server_exec);

    Some(server_path)
}

fn start_server() {
    let server_path_buf = get_server_path(None);
    let server_path = server_path_buf.unwrap();
    let server_path_str = server_path.into_os_string().into_string().unwrap();
    let command = format!("start /b {:?}", server_path_str);
    println!("Starting server with: {:?}", command);

    Command::new("cmd.exe")
        .args(["/C", command.as_str()])
        .spawn()
        .expect("Failed to start server");
}

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let _ = app;
    let is_windows = cfg!(target_os = "windows");
    let is_debug = cfg!(debug_assertions);

    start_server();
    if is_windows && !is_debug {
    }
    else {
        println!("Server not started. Debug mode is enabled or the OS is not Windows.");
    }

    Ok(())
}