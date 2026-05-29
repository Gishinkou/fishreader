mod articles;
mod commands;
mod db;
mod models;

use db::DbState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::env::temp_dir().join("fishreader"));

            let conn = db::init_db(&app_data_dir).map_err(|e| {
                eprintln!("[db] init failed: {}", e);
                e
            })?;

            // Sync built-in markdown articles from bundled resources.
            if let Ok(res_dir) = app.path().resource_dir() {
                commands::sync_builtin_articles(&conn, &res_dir);
            }

            app.manage(DbState {
                conn: Mutex::new(conn),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_articles,
            commands::get_article_content,
            commands::save_reading_progress,
            commands::get_reading_progress,
            commands::toggle_favorite,
            commands::get_settings,
            commands::update_settings,
            commands::set_kv,
            commands::get_kv,
            commands::set_always_on_top,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}