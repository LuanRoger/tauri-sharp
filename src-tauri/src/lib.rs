use proto::greeter_client::GreeterClient;
use std::process::Command;
use tonic::transport::Channel;

mod proto {
    tonic::include_proto!("greet");
}

pub async fn create_client() -> Result<GreeterClient<Channel>, Box<dyn std::error::Error>> {
    let url = "http://[::1]:5030";
    let client = GreeterClient::connect(url).await?;

    Ok(client)
}

#[tauri::command]
async fn greet(name: String) -> Result<String, String> {
    let client_result = create_client().await;
    let mut client = client_result.unwrap();

    let request_data = proto::HelloRequest { name: name.clone() };
    let request = tonic::Request::new(request_data);
    let response = client.say_hello(request).await;

    let result = match response {
        Ok(response) => {
            let response = response.into_inner();
            Ok(response.message)
        }
        Err(e) => Err(e.to_string().into()),
    };

    result
}

#[tauri::command]
fn spawn_server() {
    let mut server = Command
        ::new("C:/Users/luanr/OneDrive/Documentos/Jobs/c_sharp/TauriSharp/bin/Release/net8.0/TauriSharp-x86_64-pc-windows-msvc.exe");
    let child = server.spawn().expect("failed to execute child");
    println!("Server started with PID: {}", child.id());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, spawn_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
