use crate::player;
use crate::player::player_attributes::gen_rand_attrs;
use crate::team;
use rusqlite::{Connection, Result};
use std::{fs, path::PathBuf};

//Singleton implementation for db
pub fn init(db_path: &PathBuf) -> Result<Connection, rusqlite::Error> {
    fs::create_dir_all(db_path).expect("The app data directory should be created.");
    let sqlite_path = db_path.join("HoopDreams.sqlite");
    let conn = Connection::open(sqlite_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS teams (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            city TEXT NOT NULL UNIQUE
        );
        ",
        [],
    )?;

    let teams = team::Team::get_teams_from_db(&conn)?;
    if teams.is_empty() {
        let teams = vec![
            team::Team::new(None, "Cavaliers".to_string(), "Cleveland".to_string()),
            team::Team::new(None, "Rockets".to_string(), "Houston".to_string()),
        ];
        for mut team in teams {
            team.write_to_db(&conn)
                .expect("Database write should succeed");
        }
    }
    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
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
        "CREATE TABLE IF NOT EXISTS team_players (
            id INTEGER PRIMARY KEY,
            team_id INTEGER NOT NULL,
            player_id INTEGER NOT NULL UNIQUE
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
    let players = player::Player::get_all_players_from_db(&conn)?;
    if players.is_empty() {
        let players = vec![
            player::Player::new(
                None,
                "LeBron".to_string(),
                "James".to_string(),
                "SF".to_string(),
                37,
                6,
                250,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Stephen".to_string(),
                "Curry".to_string(),
                "PG".to_string(),
                28,
                6,
                190,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Russell".to_string(),
                "Westbrook".to_string(),
                "PG".to_string(),
                32,
                6,
                200,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Anthony".to_string(),
                "Davis".to_string(),
                "PF".to_string(),
                23,
                6,
                225,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Chris".to_string(),
                "Paul".to_string(),
                "PG".to_string(),
                33,
                6,
                175,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Kawhi".to_string(),
                "Leonard".to_string(),
                "SF".to_string(),
                29,
                6,
                225,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Nikola".to_string(),
                "Jokic".to_string(),
                "C".to_string(),
                26,
                6,
                220,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Paul".to_string(),
                "George".to_string(),
                "SF".to_string(),
                29,
                6,
                220,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Klay".to_string(),
                "Thompson".to_string(),
                "SG".to_string(),
                32,
                6,
                215,
                gen_rand_attrs(),
            ),
            player::Player::new(
                None,
                "Dwyane".to_string(),
                "Wade".to_string(),
                "SG".to_string(),
                34,
                6,
                220,
                gen_rand_attrs(),
            ),
        ];
        let teams = team::Team::get_teams_from_db(&conn)?;
        let mut i = 0;
        for mut player in players {
            player
                .write_to_db(&conn)
                .expect("Database write should succeed");
            if i % 2 == 0 {
                teams[0].add_player_to_team(&player, &conn)?;
                teams[0].add_player_to_starting_lineup(&player, &conn)?;
            } else {
                teams[1].add_player_to_team(&player, &conn)?;
                teams[1].add_player_to_starting_lineup(&player, &conn)?;
            }
            i += 1;
        }
    }

    Ok(conn)
}
