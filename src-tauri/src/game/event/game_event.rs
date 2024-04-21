use crate::game::event::jump_ball;
use crate::game::Game;
use crate::player::Player;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub has_ball: Player,
    pub action: String,
    pub time: Duration,
    pub period: u8,
    pub possession: Option<String>,
    pub starters: (Vec<Player>, Vec<Player>),
}

impl GameEvent {
    pub fn new(
        has_ball: Player,
        action: String,
        time: Duration,
        period: u8,
        possession: Option<String>,
        starters: (Vec<Player>, Vec<Player>),
    ) -> GameEvent {
        GameEvent {
            has_ball,
            action,
            time,
            period,
            possession,
            starters,
        }
    }

    pub fn generate_next_game_event(game: &mut Game, db: &Connection) -> Result<(), String> {
        if game.events.len() == 0 {
            jump_ball::generate_jump_ball(game, db)?;
        }
        let last_event = game.events.last().unwrap();
        // Generate next event
        if last_event.time.as_secs() > 0 {
            // Check if time is less than 0.3 seconds
            if last_event.time.as_secs() < Duration::from_millis(300).as_secs() {
                //Tip in event
                return Ok(());
            } else if last_event.time.as_secs() < Duration::from_millis(500).as_secs() {
                //Enough time for a shot no dribble
                return Ok(());
            } else {
                //Dribble
                return Ok(());
            }
        }
        if last_event.period >= 4 {
            if game.score.0 == game.score.1 {
                //Overtime
                return Ok(());
            }
            // Game is over
            return Ok(());
        } else {
            // End of qtr
            return Ok(());
        }
    }
}
