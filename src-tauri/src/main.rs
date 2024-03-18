// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod court;
mod player;
mod team;
mod util;

use crate::util::state::{AppState, ServiceAccess};

use tauri::{AppHandle, Manager, State};
use util::db;

#[tauri::command]
fn get_teams(app_handle: AppHandle) -> Vec<team::Team> {
    let teams = app_handle
        .db(|db| team::Team::get_teams_from_db(db))
        .unwrap();
    teams
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![get_teams])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = db::init(&handle).expect("Database initialize should succeed");

            let team1 = team::Team::new("Cavaliers".to_string(), "Cleveland".to_string());
            let team2 = team::Team::new("Rockets".to_string(), "Houston".to_string());
            team1
                .write_to_db(&db)
                .expect("Database write should succeed");
            team2
                .write_to_db(&db)
                .expect("Database write should succeed");

            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
