use rusqlite::{Connection, Result};
use std::fs;
use tauri::AppHandle;

//Singleton implementation for db
pub fn init(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("HoopDreams.sqlite");
    let conn = Connection::open(sqlite_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            position TEXT NOT NULL,
            age INTEGER NOT NULL,
            height INTEGER NOT NULL,
            weight INTEGER NOT NULL
        );
        ",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS teams (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            city TEXT NOT NULL UNIQUE
        );

        INSERT INTO teams (name, city) VALUES ('Cavaliers', 'Cleveland');

        INSERT INTO teams (name, city) VALUES ('Rockets', 'Houston');
        ",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS team_players (
            id INTEGER PRIMARY KEY,
            team_id INTEGER NOT NULL,
            player_id INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS team_bench (
            id INTEGER PRIMARY KEY,
            team_id INTEGER NOT NULL,
            player_id INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS team_starting_lineup (
            id INTEGER PRIMARY KEY,
            team_id INTEGER NOT NULL,
            player_id INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}
