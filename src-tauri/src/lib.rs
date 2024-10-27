// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod game;
mod player;
mod team;
mod util;

use crate::util::state::{AppState, ServiceAccess};

use std::sync::Arc;
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

fn simulate_game(app_handle: AppHandle, speed: u8) -> Result<(), Box<dyn std::error::Error>> {
    app_handle.emit("main", "simulation_started")?;
    let running = app_handle.state::<AppState>().running.clone();
    let state = app_handle.state::<AppState>();
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let mut game = state.game.lock().unwrap();
        let event = game
            .as_mut()
            .ok_or("Game not initialized")?
            .generate_next_game_event()?;

        match speed {
            1 => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            2 => {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            3 => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            _ => {
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }

        println!("{:?}", event);
        app_handle.emit_to("main", "game_event", event.clone())?;
        app_handle.emit_to(
            "main",
            "player_states",
            game.as_ref().unwrap().get_player_states(),
        )?;
        let game_score = game.as_ref().unwrap().get_score();
        app_handle.emit_to("main", "game_score", game_score)?;

        if event.is_game_end() {
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
    //Stop the old thread
    stop_sim(state.clone())?;
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
    let mut sim_thread = state.sim_thread.lock().unwrap();
    if sim_thread.is_some() {
        return Err("Simulation already running".to_string());
    }

    state
        .running
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let app_handle_clone = app_handle.clone();
    let running = Arc::clone(&state.running);
    *sim_thread = Some(std::thread::spawn(move || {
        let _ = simulate_game(app_handle_clone, speed);
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
    let mut sim_thread = state.sim_thread.lock().unwrap();
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
                .expect("db path should be resolved");
            let db = db::init(&path).expect("Database initialize should succeed");

            let handle = app.handle().clone();
            let app_state: State<AppState> = handle.state();
            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
