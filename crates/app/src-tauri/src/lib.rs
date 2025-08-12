static HOST: &str = "localhost:8000";
use shared::SimpleEntry;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(title: &str, body: &str) {
    let client = reqwest::blocking::Client::new();
    client
        .post(format!("http://{HOST}/push"))
        .json(&SimpleEntry {
            title: title.into(),
            body: body.into(),
        })
        .send()
        .expect("Could not send entry to server");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
