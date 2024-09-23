use crate::game::Game;
use crate::game::Possession;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub action: String,
    pub time: Duration,
    pub period: u8,
    pub possession: Possession,
}

impl GameEvent {
    pub fn new(action: String, time: Duration, period: u8, possession: Possession) -> GameEvent {
        GameEvent {
            action,
            time,
            period,
            possession,
        }
    }

    pub fn generate_next_game_event(game: &mut Game) -> Result<(), String> {
        // Generate next event
        if game.state.time.as_secs() > 0 {
            // Check if time is less than 0.3 seconds
            if game.state.time.as_secs() < Duration::from_millis(300).as_secs() {
                //Tip in event
                println!("Tip in");
                game.state.time = Duration::from_secs(0);
                return Ok(());
            } else if game.state.time.as_secs() < Duration::from_millis(500).as_secs() {
                //Enough time for a shot no dribble
                game.state.time = Duration::from_secs(0);
                return Ok(());
            } else {
                //Default
                game.handle_player_actions();
                //Generate random number between 1 and 24 float
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0.0..12.0);
                if random > game.state.time.as_secs_f32() {
                    game.state.time = Duration::from_secs(0);
                } else {
                    game.state.time = game.state.time - Duration::from_secs_f32(random);
                }

                return Ok(());
            }
        } else if game.state.period >= 4 {
            if game.state.score.0 == game.state.score.1 {
                //Overtime
                game.state.period += 1;
                game.state.time = Duration::from_secs(300);
                return Ok(());
            }
            // Game is over
            return Ok(());
        } else {
            // End of qtr
            game.state.period += 1;
            game.state.time = Duration::from_secs(720);
            return Ok(());
        }
    }
}
