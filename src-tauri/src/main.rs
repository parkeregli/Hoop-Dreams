// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod court;
mod player;
mod team;
mod util;

use court::Court;
use team::Team;
use util::db;

fn main() {
    let db = db::init().expect("Could not connect to database");

    let _court = Court::new();

    let teams = Team::get_teams_from_db(&db).unwrap();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
