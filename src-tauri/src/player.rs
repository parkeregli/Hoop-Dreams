use std::fmt;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    first_name: String,
    last_name: String,
    position: String,
    age: u8,
    height: u8,
    weight: u16,
}

impl Player {
    pub fn new(
        first_name: String,
        last_name: String,
        position: String,
        age: u8,
        height: u8,
        weight: u16,
    ) -> Player {
        Player {
            first_name,
            last_name,
            position,
            age,
            height,
            weight,
        }
    }

    pub fn write_to_db(&self, conn: &Connection) -> rusqlite::Result<()> {
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
        Ok(())
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
