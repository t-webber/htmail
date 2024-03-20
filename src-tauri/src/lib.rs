// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn send_mail(from: &str, to: &str, subject: &str, body: &str) -> String {
    println!("IN: {from} => {to} => {subject} =>\n{body}\n!!!");
    format!("[WARNING] bobbob\n[FAILURE] bobbobbob")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![send_mail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
