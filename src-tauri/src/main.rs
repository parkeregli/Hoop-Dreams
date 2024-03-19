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

            let players = vec![
                player::Player::new(
                    "LeBron".to_string(),
                    "James".to_string(),
                    "SF".to_string(),
                    37,
                    6,
                    250,
                ),
                player::Player::new(
                    "Stephen".to_string(),
                    "Curry".to_string(),
                    "PG".to_string(),
                    28,
                    6,
                    190,
                ),
                player::Player::new(
                    "Russell".to_string(),
                    "Westbrook".to_string(),
                    "PG".to_string(),
                    32,
                    6,
                    200,
                ),
                player::Player::new(
                    "Anthony".to_string(),
                    "Davis".to_string(),
                    "PF".to_string(),
                    23,
                    6,
                    225,
                ),
                player::Player::new(
                    "Chris".to_string(),
                    "Paul".to_string(),
                    "PG".to_string(),
                    33,
                    6,
                    175,
                ),
                player::Player::new(
                    "Kawhi".to_string(),
                    "Leonard".to_string(),
                    "SF".to_string(),
                    29,
                    6,
                    225,
                ),
                player::Player::new(
                    "Nikola".to_string(),
                    "Jokic".to_string(),
                    "C".to_string(),
                    26,
                    6,
                    220,
                ),
                player::Player::new(
                    "Paul".to_string(),
                    "George".to_string(),
                    "SF".to_string(),
                    29,
                    6,
                    220,
                ),
                player::Player::new(
                    "Klay".to_string(),
                    "Thompson".to_string(),
                    "SG".to_string(),
                    32,
                    6,
                    215,
                ),
                player::Player::new(
                    "Dwyane".to_string(),
                    "Wade".to_string(),
                    "SG".to_string(),
                    34,
                    6,
                    220,
                ),
            ];

            for player in players {
                player
                    .write_to_db(&db)
                    .expect("Database write should succeed");
            }

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
