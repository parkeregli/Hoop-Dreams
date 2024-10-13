use crate::game::Game;
use crate::game::Possession;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub action: String,
    pub time: String,
    pub period: u8,
    pub possession: Option<Possession>,
}

impl GameEvent {
    pub fn new(
        action: String,
        time: String,
        period: u8,
        possession: Option<Possession>,
    ) -> GameEvent {
        GameEvent {
            action,
            time,
            period,
            possession,
        }
    }

    pub fn is_game_end(&self) -> bool {
        self.action == "End of Game"
    }

    pub fn generate_next_game_event(game: &mut Game) -> Result<GameEvent, String> {
        // Generate next event
        if game.state.time.as_secs() > 0 {
            /*
                        // Check if time is less than 0.3 seconds
                        if game.state.time <= Duration::from_millis(300) {
                            //Tip in event
                            println!("Tip in");
                            game.handle_player_actions();
                            game.state.time = Duration::from_secs(0);
                            return Ok(());
                        } else if game.state.time <= Duration::from_millis(500) {
                            //Enough time for a shot no dribble
                            println!("Shot no dribble");
                            game.handle_player_actions();
                            game.state.time = Duration::from_secs(0);
                            return Ok(());
                        } else {
            */
            //Default
            let event = game.handle_player_actions();
            //Generate random number between 1 and 24 float
            let mut rng = rand::thread_rng();
            let max = f32::min(8.0, game.state.time.as_secs_f32());
            let random = rng.gen_range(1.0..max);
            if random > game.state.shot_clock.as_secs_f32() {
                println!("Shot clock ran out. Turnover!");
                match game.state.possession {
                    Some((Possession::Home, _)) => {
                        game.change_possession(Some((Possession::Away, 3)));
                    }
                    Some((Possession::Away, _)) => {
                        game.change_possession(Some((Possession::Home, 3)));
                    }
                    None => {
                        //Jump ball
                    }
                }
                return Ok(GameEvent::new(
                    "Turnover".to_string(),
                    game.get_time(),
                    0,
                    None,
                ));
            } else {
                game.state.shot_clock -= Duration::from_secs_f32(random)
            }
            game.state.time -= Duration::from_secs_f32(random);

            return Ok(event.unwrap());
        //    }
        } else if game.state.period >= 4 {
            if game.state.score.0 == game.state.score.1 {
                //Overtime
                game.state.period += 1;
                game.state.time = Duration::from_secs(300);
                let event = game.handle_player_actions();
                //Generate random number between 1 and 24 float
                let mut rng = rand::thread_rng();
                let max = f32::min(6.0, game.state.time.as_secs_f32());
                let random = rng.gen_range(1.0..max);
                if random > game.state.shot_clock.as_secs_f32() {
                    println!("Shot clock ran out. Turnover!");
                    match game.state.possession {
                        Some((Possession::Home, _)) => {
                            game.change_possession(Some((Possession::Away, 3)));
                        }
                        Some((Possession::Away, _)) => {
                            game.change_possession(Some((Possession::Home, 3)));
                        }
                        None => {
                            //Jump ball
                        }
                    }
                } else {
                    game.state.shot_clock -= Duration::from_secs_f32(random)
                }
                game.state.time -= Duration::from_secs_f32(random);
                game.events.push(event.clone().unwrap());

                return Ok(event.unwrap());
            }
            // Game is over
            game.state.time = Duration::from_secs(0);
            return Ok(GameEvent::new(
                "End of Game".to_string(),
                game.get_time(),
                0,
                None,
            ));
        } else {
            // End of qtr
            game.state.period += 1;
            game.state.time = Duration::from_secs(720);
            return Ok(GameEvent::new(
                "End of Quarter".to_string(),
                game.get_time(),
                0,
                None,
            ));
        }
    }
}
