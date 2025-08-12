use shared::SimpleEntry;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn push(title: String, body: String) {
    let client = reqwest::blocking::Client::new();
    client
        .post(format!("http://localhost:8000/push"))
        .json(&SimpleEntry {
            title: title,
            body: body,
        })
        .send()
        .expect("AAAAA");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![push])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
