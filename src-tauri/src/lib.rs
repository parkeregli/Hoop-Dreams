// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod game;
mod player;
mod team;
mod util;

use crate::util::state::{AppState, ServiceAccess};

use tauri::{AppHandle, Emitter, Listener, Manager, State};
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

fn simulate_game(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    app_handle.emit("main", "simulation_started")?;
    app_handle.game_mut(|game| {
        let mut game_end = false;
        while !game_end {
            let event = game.generate_next_game_event().unwrap();
            println!("{:?}", event);
            let _ = app_handle.emit_to("main", "game_event", event.clone());
            game_end = event.is_game_end();
        }
    });
    app_handle.emit("main", "simulation_ended")?;
    Ok(())
}

#[tauri::command]
fn start_sim(app_handle: AppHandle) -> Result<(), String> {
    std::thread::spawn(move || {
        let _ = simulate_game(app_handle);
    });
    Ok(())
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
            start_sim,
        ])
        .setup(|app| {
            let webview = app.get_webview_window("main").unwrap();
            let path = app
                .path()
                .resolve("db", tauri::path::BaseDirectory::Config)
                .expect("db path should be resolved");
            let db = db::init(&path).expect("Database initialize should succeed");

            let handle = app.handle().clone();
            let app_state: State<AppState> = handle.state();
            *app_state.db.lock().unwrap() = Some(db);

            let handle = app.handle().clone();
            webview.listen("start_sim", move |_| {
                let state = handle.state::<AppState>();
                let mut game = state.game.lock().unwrap();
                let _ = game.as_mut().unwrap().generate_next_game_event();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
