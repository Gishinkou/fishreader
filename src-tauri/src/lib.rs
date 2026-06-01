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

            // Sync built-in markdown articles.
            // 1) Production / packaged: read from Tauri resource_dir (articles/ subdir).
            // 2) Dev fallback: if the bundled articles dir doesn't exist (running via
            //    `cargo run` or `tauri dev`'s target/debug), fall back to the source
            //    directory at compile time (CARGO_MANIFEST_DIR/resources/articles).
            let mut synced = false;
            if let Ok(res_dir) = app.path().resource_dir() {
                let articles_dir = res_dir.join("articles");
                if articles_dir.exists() {
                    commands::sync_builtin_articles(&conn, &res_dir);
                    synced = true;
                }
            }
            if !synced {
                let dev_res_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("resources");
                if dev_res_dir.join("articles").exists() {
                    commands::sync_builtin_articles(&conn, &dev_res_dir);
                } else {
                    eprintln!(
                        "[articles] no built-in articles directory found (checked resource_dir and {:?})",
                        dev_res_dir
                    );
                }
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
            commands::add_note,
            commands::list_notes,
            commands::delete_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}