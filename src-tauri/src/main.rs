#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/*  I have not touched the rust side (yet) because i'm still learning rust :) */


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    tauri::Builder::default()
    .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                  match handle
                    .updater()
                    .unwrap()
                    .check()
                    .await
                  {
                    Ok(update) => {
                       update.download_and_install().await;
                    }
                    Err(e) => {
                      println!("ERROR: {}", e);
                    }
                  }
                });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
