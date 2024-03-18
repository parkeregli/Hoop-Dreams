use rusqlite::Connection;
use std::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Team {
    name: String,
    city: String,
}

impl Team {
    pub fn new(name: String, city: String) -> Team {
        Team { name, city }
    }

    pub fn write_to_db(&self, db: &Connection) -> Result<(), rusqlite::Error> {
        let mut stmt = db.prepare("INSERT OR IGNORE INTO teams (name, city) VALUES (?, ?)")?;
        stmt.execute([&self.name, &self.city])?;

        Ok(())
    }

    pub fn get_teams_from_db(db: &Connection) -> Result<Vec<Team>, rusqlite::Error> {
        let mut stmt = db.prepare("SELECT name, city FROM teams")?;
        let teams: Vec<Team> = stmt
            .query_map([], |row| {
                Ok(Team {
                    name: row.get(0)?,
                    city: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<Team>, _>>()?;
        Ok(teams)
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Team: {}, City: {}", self.name, self.city)?;
        Ok(())
    }
}
