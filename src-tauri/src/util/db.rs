use rusqlite::{Connection, Result};

//Singleton implementation for db
pub fn init() -> Result<Connection> {
    let conn = Connection::open("HoopDreams.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            position TEXT NOT NULL,
            age INTEGER NOT NULL,
            height INTEGER NOT NULL,
            weight INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS teams (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            city TEXT NOT NULL UNIQUE
        )",
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
