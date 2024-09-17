use crate::player::{self, player_attributes::gen_rand_attrs};
use rusqlite::Connection;
use std::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Team {
    id: Option<i64>,
    name: String,
    city: String,
}

impl Team {
    pub fn new(id: Option<i64>, name: String, city: String) -> Team {
        Team { id, name, city }
    }

    pub fn write_to_db(&mut self, db: &Connection) -> Result<(), rusqlite::Error> {
        let mut stmt = db.prepare("INSERT OR IGNORE INTO teams (name, city) VALUES (?, ?)")?;
        stmt.execute([&self.name, &self.city])?;

        let id = db.last_insert_rowid();
        self.id = Some(id);
        Ok(())
    }

    pub fn get_teams_from_db(db: &Connection) -> Result<Vec<Team>, rusqlite::Error> {
        let mut stmt = db.prepare("SELECT id, name, city FROM teams")?;
        let teams: Vec<Team> = stmt
            .query_map([], |row| {
                Ok(Team {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    city: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<Team>, _>>()?;
        Ok(teams)
    }

    pub fn get_team(team_id: &i64, db: &Connection) -> Result<Team, rusqlite::Error> {
        let mut stmt = db.prepare("SELECT id, name, city FROM teams WHERE id = ?")?;
        let team = stmt.query_row([team_id], |row| {
            Ok(Team {
                id: row.get(0)?,
                name: row.get(1)?,
                city: row.get(2)?,
            })
        })?;
        Ok(team)
    }

    pub fn add_player_to_team(
        &self,
        player: &player::Player,
        db: &Connection,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = db.prepare("INSERT INTO team_players (team_id, player_id) VALUES (?, ?)")?;
        stmt.execute([self.id.unwrap(), player.get_id().unwrap()])?;
        Ok(())
    }

    pub fn add_player_to_starting_lineup(
        &self,
        player: &player::Player,
        db: &Connection,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt =
            db.prepare("INSERT INTO team_starting_lineup (team_id, player_id) VALUES (?, ?)")?;
        stmt.execute([self.id.unwrap(), player.get_id().unwrap()])?;
        Ok(())
    }

    pub fn get_starting_lineup(
        &self,
        db: &Connection,
    ) -> Result<[player::Player; 5], rusqlite::Error> {
        let mut stmt = db.prepare("
            SELECT players.id, players.first_name, players.last_name, players.position, players.age, players.height, players.weight
            FROM team_starting_lineup
            INNER JOIN players ON team_starting_lineup.player_id = players.id
            WHERE team_id = ?")?;
        let players: [player::Player; 5] = stmt
            .query_map([self.id], |row| {
                Ok(player::Player::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    gen_rand_attrs(),
                ))
            })?
            .collect::<Result<Vec<player::Player>, _>>()?
            .try_into()
            //TODO: Add error handling
            .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;
        Ok(players)
    }

    pub fn get_bench(&self, db: &Connection) -> Result<Vec<player::Player>, rusqlite::Error> {
        let mut stmt = db.prepare("
            SELECT players.id, players.first_name, players.last_name, players.position, players.age, players.height, players.weight
            FROM team_bench
            INNER JOIN players ON team_bench.player_id = players.id
            WHERE team_id = ?")?;
        let players: Vec<player::Player> = stmt
            .query_map([self.id], |row| {
                Ok(player::Player::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    gen_rand_attrs(),
                ))
            })?
            .collect::<Result<Vec<player::Player>, _>>()?;
        Ok(players)
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Team: {}, City: {}", self.name, self.city)?;
        Ok(())
    }
}
