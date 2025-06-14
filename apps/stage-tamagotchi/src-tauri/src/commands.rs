use std::path::Path;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_settings_window(app: tauri::AppHandle) {
  if let Some(window) = app.get_webview_window("settings") {
    let _ = window.show();
    return;
  }

  let _ = WebviewWindowBuilder::new(
    &app,
    "settings",
    WebviewUrl::App(Path::new("#/settings").to_path_buf()),
  )
  .title("settings")
  .inner_size(600.0, 800.0)
  .build()
  .unwrap();
}

#[tauri::command]
pub async fn open_chat_window(app: tauri::AppHandle) {
  if let Some(window) = app.get_webview_window("chat") {
    let _ = window.show();
    return;
  }

  let _ = WebviewWindowBuilder::new(
    &app,
    "chat",
    WebviewUrl::App(Path::new("#/chat").to_path_buf()),
  )
  .title("chat")
  .inner_size(600.0, 800.0)
  .build()
  .unwrap();
}
