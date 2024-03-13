// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod court;
mod player;
mod team;
mod util;

use court::Court;
use player::Player;
use team::Team;
use util::db;

fn main() {
    let db = db::init().unwrap();
    let version: String = db
        .query_row("SELECT sqlite_version()", [], |row| row.get(0))
        .unwrap();

    println!("Sqlite version: {}", version);

    let _court = Court::new();

    let mut team = Team::new("Cavs", "Cleveland");

    team.add_player_to_starting_lineup(
        Player::new(
            "LeBron James",
            "Small Forward",
            37,
            6,
            250,
            92,
            90,
            88,
            80,
            80,
            80,
        ),
        0,
    );

    team.add_player_to_bench(
        Player::new(
            "Stephen Curry",
            "Point Guard",
            32,
            6,
            190,
            85,
            90,
            88,
            80,
            80,
            80,
        ),
        0,
    );

    println!("{:?}", team);

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
