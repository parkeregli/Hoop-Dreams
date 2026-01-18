// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod game;
mod player;
mod team;
mod util;

use crate::util::state::{AppState, ServiceAccess};

use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};
use util::db;

#[tauri::command]
fn get_teams(app_handle: AppHandle) -> Result<Vec<team::Team>, String> {
    let teams = app_handle
        .db(|db| team::Team::get_teams_from_db(db))
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(teams)
}

#[tauri::command]
fn get_team(app_handle: AppHandle, team_id: i64) -> Result<team::Team, String> {
    let team = app_handle
        .db(|db| team::Team::get_team(&team_id, db))
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(team)
}

#[tauri::command]
fn get_team_starting_lineup(app_handle: AppHandle, team_id: i64) -> Result<[player::Player; 5], String> {
    let team = app_handle
        .db(|db| team::Team::get_team(&team_id, db))
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    let players = app_handle
        .db(|db| team.get_starting_lineup(db))
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(players)
}

#[tauri::command]
fn load_game(app_handle: AppHandle, state: tauri::State<AppState>) -> Result<game::Game, String> {
    let new_game = app_handle
        .db(|db| game::Game::new(&db))
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    let mut game_guard = state.game.lock().map_err(|e| e.to_string())?;
    *game_guard = Some(new_game.clone());
    Ok(new_game)
}

fn simulate_game(app_handle: AppHandle, speed: u8) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    app_handle.emit("main", "simulation_started")?;
    let running = app_handle.state::<AppState>().running.clone();
    let state = app_handle.state::<AppState>();
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let mut game_guard = state.game.lock().map_err(|e| format!("Failed to acquire game lock: {}", e))?;
        let game = game_guard
            .as_mut()
            .ok_or("Game not initialized")?;
        let event = game.generate_next_game_event()?;
        let player_states = game.get_player_states();
        let game_score = game.get_score();
        let game_clock = game.get_clock();
        let is_game_end = event.is_game_end();
        drop(game_guard);

        match speed {
            1 => {
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            2 => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            3 => {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            _ => {
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        }

        println!("{:?}", event);
        app_handle.emit_to("main", "game_event", event)?;
        app_handle.emit_to("main", "player_states", player_states)?;
        app_handle.emit_to("main", "game_score", game_score)?;
        app_handle.emit_to("main", "game_clock", game_clock)?;

        if is_game_end {
            break;
        }
    }
    app_handle.emit("main", "simulation_ended")?;
    Ok(())
}

#[tauri::command]
fn set_sim_speed(
    app_handle: AppHandle,
    state: tauri::State<AppState>,
    speed: u8,
) -> Result<(), String> {
    //Stop the old thread (ignore error if not running)
    let _ = stop_sim(state.clone());
    //Start the new thread
    state
        .running
        .store(true, std::sync::atomic::Ordering::SeqCst);
    if speed > 0 {
        start_sim(app_handle.clone(), state.clone(), speed)?;
    }
    Ok(())
}

#[tauri::command]
fn start_sim(
    app_handle: AppHandle,
    state: tauri::State<AppState>,
    speed: u8,
) -> Result<(), String> {
    let mut sim_thread = state
        .sim_thread
        .lock()
        .map_err(|e| format!("Failed to acquire sim thread lock: {}", e))?;
    if sim_thread.is_some() {
        return Err("Simulation already running".to_string());
    }

    state
        .running
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let app_handle_clone = app_handle.clone();
    let running = Arc::clone(&state.running);
    *sim_thread = Some(std::thread::spawn(move || {
        if let Err(e) = simulate_game(app_handle_clone, speed) {
            eprintln!("Simulation error: {}", e);
        }
        running.store(false, std::sync::atomic::Ordering::SeqCst);
    }));
    Ok(())
}

#[tauri::command]
fn stop_sim(state: tauri::State<AppState>) -> Result<(), String> {
    if !state
        .running
        .swap(false, std::sync::atomic::Ordering::SeqCst)
    {
        return Err("Simulation not running".to_string());
    }
    let mut sim_thread = state
        .sim_thread
        .lock()
        .map_err(|e| format!("Failed to acquire sim thread lock: {}", e))?;
    if let Some(handle) = sim_thread.take() {
        handle
            .join()
            .map_err(|_| "Failed to join simulation thread".to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            game: Default::default(),
            running: Default::default(),
            sim_thread: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            get_teams,
            get_team_starting_lineup,
            get_team,
            load_game,
            start_sim,
            stop_sim,
            set_sim_speed
        ])
        .setup(|app| {
            let path = app
                .path()
                .resolve("db", tauri::path::BaseDirectory::Config)
                .map_err(|e| format!("Failed to resolve db path: {}", e))?;
            let db = db::init(&path)
                .map_err(|e| format!("Failed to initialize database: {}", e))?;

            let handle = app.handle().clone();
            let app_state: State<AppState> = handle.state();
            let mut db_guard = app_state
                .db
                .lock()
                .map_err(|e| format!("Failed to acquire db lock: {}", e))?;
            *db_guard = Some(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
