// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod court;
mod game;
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

#[tauri::command]
fn get_team(app_handle: AppHandle, team_id: i64) -> team::Team {
    let team = app_handle
        .db(|db| team::Team::get_team(&team_id, db))
        .unwrap();
    team
}

#[tauri::command]
fn get_team_starting_lineup(app_handle: AppHandle, team_id: i64) -> Vec<player::Player> {
    let team = app_handle
        .db(|db| team::Team::get_team(&team_id, db))
        .unwrap();
    let players = app_handle.db(|db| team.get_starting_lineup(db)).unwrap();
    players
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            get_teams,
            get_team_starting_lineup,
            get_team
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = db::init(&handle).expect("Database initialize should succeed");

            let teams = team::Team::get_teams_from_db(&db).unwrap();
            let home_team = &teams[0];
            let away_team = &teams[1];
            let mut game = game::Game::new(home_team, away_team).unwrap();

            let _ = game.generate_next_game_event(&db).unwrap();

            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
