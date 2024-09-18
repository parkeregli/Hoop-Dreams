use crate::game::Game;
use crate::game::{event::jump_ball, Possession};
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
        if game.events.len() == 0 {
            let _ = jump_ball::generate_jump_ball(game);
        }
        let last_event = game.events.last().unwrap();
        // Generate next event
        println!("Last event time: {:?}", last_event.time.as_secs());
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
                for (_, p) in game.state.player_states.0.iter_mut().enumerate() {
                    p.generate_next_player_state(
                        p.has_ball,
                        game.state.possession == Possession::Home,
                    );
                }
                for (_, p) in game.state.player_states.1.iter_mut().enumerate() {
                    p.generate_next_player_state(
                        p.has_ball,
                        game.state.possession == Possession::Away,
                    );
                }

                return Ok(());
            }
        }
        if last_event.period >= 4 {
            if game.state.score.0 == game.state.score.1 {
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
