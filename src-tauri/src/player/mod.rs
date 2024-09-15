pub mod player_attributes;
pub mod player_state;
pub mod player_stats;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    position: String,
    age: u8,
    height: i32,
    weight: i32,
    attributes: player_attributes::PlayerAttributes,
}

impl Player {
    pub fn new(
        id: Option<i64>,
        first_name: String,
        last_name: String,
        position: String,
        age: u8,
        height: i32,
        weight: i32,
        attributes: player_attributes::PlayerAttributes,
    ) -> Player {
        Player {
            id,
            first_name,
            last_name,
            position,
            age,
            height,
            weight,
            attributes,
        }
    }

    pub fn get_id(&self) -> Result<i64, String> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(String::from("Player has no id. Write to db first."))
        }
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn write_to_db(&mut self, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO players (first_name, last_name, position, age, height, weight) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.first_name,
                self.last_name,
                self.position,
                self.age,
                self.height,
                self.weight,
            ],
        )?;
        let id = conn.last_insert_rowid();
        self.id = Some(id);
        Ok(())
    }

    pub fn get_all_players_from_db(db: &Connection) -> Result<Vec<Player>, rusqlite::Error> {
        let mut stmt = db.prepare(
            "SELECT id, first_name, last_name, position, age, height, weight FROM players",
        )?;
        let players: Vec<Player> = stmt
            .query_map([], |row| {
                Ok(Player {
                    id: row.get(0)?,
                    first_name: row.get(1)?,
                    last_name: row.get(2)?,
                    position: row.get(3)?,
                    age: row.get(4)?,
                    height: row.get(5)?,
                    weight: row.get(6)?,
                    attributes: player_attributes::gen_rand_attrs(),
                })
            })?
            .collect::<Result<Vec<Player>, _>>()?;
        Ok(players)
    }

    pub fn get_player_attributes(&self) -> player_attributes::PlayerAttributes {
        self.attributes.clone()
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {} {}, Position: {}, Age: {}, Height: {}, Weight: {}",
            self.first_name, self.last_name, self.position, self.age, self.height, self.weight
        )
    }
}
