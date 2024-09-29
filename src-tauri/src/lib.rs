// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
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
fn get_team_starting_lineup(app_handle: AppHandle, team_id: i64) -> [player::Player; 5] {
    let team = app_handle
        .db(|db| team::Team::get_team(&team_id, db))
        .unwrap();
    let players = app_handle.db(|db| team.get_starting_lineup(db)).unwrap();
    players
}

#[tauri::command]
fn load_game(app_handle: AppHandle, state: tauri::State<AppState>) -> game::Game {
    let new_game = app_handle.db(|db| game::Game::new(&db).unwrap());
    *state.game.lock().unwrap() = Some(new_game.clone());
    return new_game;
}

#[tauri::command]
fn next_play(app_handle: AppHandle) {
    app_handle.game_mut(|game| {
        let _ = game.generate_next_game_event();
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            game: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            get_teams,
            get_team_starting_lineup,
            get_team,
            load_game,
            next_play
        ])
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();
            let path = app
                .path()
                .resolve("db", tauri::path::BaseDirectory::Config)
                .expect("db path should be resolved");
            let db = db::init(&path).expect("Database initialize should succeed");

            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
