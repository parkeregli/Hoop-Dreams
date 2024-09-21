use crate::game::Game;
use crate::game::{event::jump_ball, Possession};
use crate::player::player_state::{PlayerAction, PlayerState};
use crate::player::Player;
use rand::{thread_rng, Rng};
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
            return Ok(());
        }
        let last_event = game.events.last().unwrap();
        // Generate next event
        println!("Last event time: {:?}", last_event.time.as_secs());
        if game.state.time.as_secs() > 0 {
            // Check if time is less than 0.3 seconds
            if game.state.time.as_secs() < Duration::from_millis(300).as_secs() {
                //Tip in event
                return Ok(());
            } else if game.state.time.as_secs() < Duration::from_millis(500).as_secs() {
                //Enough time for a shot no dribble
                return Ok(());
            } else {
                //Default
                if game.state.possession == Possession::Home {
                    let player_with_ball_index = game
                        .state
                        .team_state
                        .0
                        .active_players
                        .iter()
                        .position(|p| p.1.has_ball)
                        .unwrap();

                    let is_shot = game.state.team_state.0.active_players[player_with_ball_index]
                        .1
                        .is_shot();
                    if is_shot.0 {
                        //Random 50 percent chance of a make
                        let random = rand::random::<f32>();
                        if random < 0.5 {
                            //Shot made
                            println!("Shot Made");
                            game.state.score.0 += is_shot.1;
                        } else {
                            println!("Shot Missed");
                        }
                        game.state.possession = Possession::Away;
                        let mut rng = thread_rng();
                        let player_index = rng.gen_range(0..5);
                        game.state.team_state.0.active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                        game.state.team_state.1.active_players[player_index]
                            .1
                            .has_ball = true;
                    }
                    if game.state.team_state.0.active_players[player_with_ball_index]
                        .1
                        .action
                        == PlayerAction::Pass
                    {
                        let mut rng = thread_rng();
                        let random_index = rng.gen_range(0..5);
                        game.state.team_state.0.active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                        game.state.team_state.0.active_players[random_index]
                            .1
                            .has_ball = true;
                    }
                } else if game.state.possession == Possession::Away {
                    let player_with_ball_index = game
                        .state
                        .team_state
                        .1
                        .active_players
                        .iter()
                        .position(|p| p.1.has_ball)
                        .unwrap();
                    let is_shot = game.state.team_state.1.active_players[player_with_ball_index]
                        .1
                        .is_shot();
                    if is_shot.0 {
                        // Random 50 percent chance of a make
                        let random = rand::random::<f32>();
                        if random < 0.5 {
                            // Shot made
                            println!("Shot Made");
                            game.state.score.1 += is_shot.1; // Changed to .1 for away team score
                        } else {
                            println!("Shot Missed");
                        }
                        game.state.possession = Possession::Home;
                        let mut rng = thread_rng();
                        let player_index = rng.gen_range(0..5);
                        game.state.team_state.1.active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                        game.state.team_state.0.active_players[player_index]
                            .1
                            .has_ball = true;
                    }
                    if game.state.team_state.1.active_players[player_with_ball_index]
                        .1
                        .action
                        == PlayerAction::Pass
                    {
                        let mut rng = thread_rng();
                        let random_index = rng.gen_range(0..5);
                        game.state.team_state.1.active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                        game.state.team_state.1.active_players[random_index]
                            .1
                            .has_ball = true;
                    }
                }
                //Generating new state for all players
                for (_, p) in game
                    .state
                    .team_state
                    .0
                    .active_players
                    .iter_mut()
                    .enumerate()
                {
                    p.1.generate_next_player_state(
                        game.state.possession == Possession::Home,
                        p.1.has_ball,
                    );
                }
                for (_, p) in game
                    .state
                    .team_state
                    .1
                    .active_players
                    .iter_mut()
                    .enumerate()
                {
                    p.1.generate_next_player_state(
                        game.state.possession == Possession::Away,
                        p.1.has_ball,
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
