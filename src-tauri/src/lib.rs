use proto::greeter_client::GreeterClient;
use proto::HelloRequest;
use tonic::transport::Channel;

pub mod connection;
pub mod proto;

#[tauri::command]
async fn greet(name: String) -> Result<String, String> {
    let mut client = connect_using!(GreeterClient, None).await.unwrap();

    let request_data = HelloRequest { name: name.clone() };
    let response = client.say_hello(request_data).await;

    let result = match response {
        Ok(response) => {
            let response = response.into_inner();
            Ok(response.message)
        }
        Err(e) => Err(e.to_string().into()),
    };

    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
