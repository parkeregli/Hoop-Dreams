use rusqlite::Connection;

pub mod event;
use crate::game::event::game_event;
use crate::team::Team;

pub struct Game<'a> {
    teams: (&'a Team, &'a Team),
    score: (u32, u32),
    fouls: (u8, u8),
    timeouts: (u8, u8),
    events: Vec<game_event::GameEvent>,
}

impl Game<'_> {
    pub fn new<'a>(home_team: &'a Team, away_team: &'a Team) -> Result<Game<'a>, rusqlite::Error> {
        Ok(Game {
            teams: (home_team, away_team),
            score: (0, 0),
            fouls: (0, 0),
            timeouts: (0, 0),
            events: Vec::new(),
        })
    }

    pub fn generate_next_game_event(&mut self, db: &Connection) -> Result<(), String> {
        let _ = game_event::GameEvent::generate_next_game_event(self, db)?;
        Ok(())
    }
}
