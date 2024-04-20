use crate::game::event::jump_ball;
use crate::game::Game;
use crate::player;
use rand::Rng;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub action: String,
    pub time: Duration,
    pub period: u8,
    pub possession: Option<String>,
    pub players: (Vec<player::Player>, Vec<player::Player>),
}

impl GameEvent {
    pub fn new(
        action: String,
        time: Duration,
        period: u8,
        possession: Option<String>,
        players: (Vec<player::Player>, Vec<player::Player>),
    ) -> GameEvent {
        GameEvent {
            action,
            time,
            period,
            possession,
            players,
        }
    }

    pub fn rng(home_value: i32, away_value: i32) -> u8 {
        println!("Home: {} Away: {}", home_value, away_value);
        // 0 = home, 1 = away
        let sub = home_value - away_value;
        let abs_sub = sub.abs();
        let ratio = abs_sub + 50;
        let mut rng = rand::thread_rng();
        let winner = rng.gen_range(0..100);

        println!("Sub: {}", sub);
        println!("Ratio: {}", ratio);
        println!("Rand: {}", winner);

        if sub == 0 {
            if winner < 50 {
                0
            } else {
                1
            }
        } else if sub > 0 {
            if winner < ratio {
                0
            } else {
                1
            }
        } else {
            if winner < ratio {
                1
            } else {
                0
            }
        }
    }

    pub fn generate_next_game_event(game: &mut Game, db: &Connection) -> Result<(), String> {
        if game.events.len() == 0 {
            jump_ball::generate_jump_ball(game, db)?;
        }
        return Ok(());
        /*
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
                game.events.push(GameEvent::new(
                    "Overtime".to_string(),
                    Duration::from_secs(60 * 5),
                    last_event.period + 1,
                    (
                        game.teams
                            .0
                            .get_starting_lineup(db)
                            .expect("Error getting starting lineups"),
                        game.teams
                            .1
                            .get_starting_lineup(db)
                            .expect("Error getting starting lineups"),
                    ),
                    (
                        game.teams.0.get_bench(db).expect("Error getting bench"),
                        game.teams.1.get_bench(db).expect("Error getting bench"),
                    ),
                ));

                return Ok(());
            }
            //Game is over
            game.events.push(GameEvent::new(
                "Game Over".to_string(),
                Duration::from_secs(0),
                0,
                last_event.starting_lineups,
                last_event.bench,
            ));
            return Ok(());
        } else {
            game.events.push(GameEvent::new(
                "End of Period".to_string(),
                Duration::from_secs(60 * 5),
                last_event.period + 1,
                last_event.starting_lineups,
                last_event.bench,
            ));
            return Ok(());
        }
        */
    }
}
