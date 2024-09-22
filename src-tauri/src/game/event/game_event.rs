use crate::game::Game;
use crate::game::Possession;
use crate::player::player_state::PlayerAction;
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
                if game.state.possession == Possession::Home {
                    let player_with_ball_index = game.state.team_state[0]
                        .active_players
                        .iter()
                        .position(|p| p.1.has_ball)
                        .unwrap();

                    let is_shot = game.state.team_state[0].active_players[player_with_ball_index]
                        .1
                        .is_shot();
                    println!(
                        "Shot: {:?}, Area: {:?}",
                        is_shot,
                        game.state.team_state[0].active_players[player_with_ball_index]
                            .1
                            .current_area
                    );
                    if is_shot.0 {
                        let random = thread_rng().gen_range(0.0..1.0);
                        println!(
                            "RNG: {}, Shot Chance: {}",
                            random,
                            game.state.team_state[0].active_players[player_with_ball_index]
                                .1
                                .calculate_shot_chance(
                                    game.state.team_state[0].active_players[player_with_ball_index]
                                        .0
                                        .attributes()
                                )
                        );
                        if random
                            < game.state.team_state[0].active_players[player_with_ball_index]
                                .1
                                .calculate_shot_chance(
                                    game.state.team_state[0].active_players[player_with_ball_index]
                                        .0
                                        .attributes(),
                                )
                        {
                            //Shot made
                            println!(
                                "{} shoots from {:?} and makes it!",
                                game.state.team_state[0].active_players[player_with_ball_index]
                                    .0
                                    .first_name,
                                game.state.team_state[0].active_players[player_with_ball_index]
                                    .1
                                    .current_area
                            );
                            game.state.score.0 += is_shot.1;
                        } else {
                            println!(
                                "{} shoots from {:?} and misses it!",
                                game.state.team_state[0].active_players[player_with_ball_index]
                                    .0
                                    .first_name,
                                game.state.team_state[0].active_players[player_with_ball_index]
                                    .1
                                    .current_area
                            );
                        }
                        game.change_possession(Possession::Away);
                        let mut rng = thread_rng();
                        let player_index = rng.gen_range(0..5);
                        game.state.team_state[0].active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                        game.state.team_state[1].active_players[player_index]
                            .1
                            .has_ball = true;
                    }
                    if game.state.team_state[0].active_players[player_with_ball_index]
                        .1
                        .action
                        == PlayerAction::Pass
                    {
                        let mut rng = thread_rng();
                        let player_count = game.state.team_state[0].active_players.len();

                        // Collect indices of players without the ball
                        let players_without_ball: Vec<usize> = (0..player_count)
                            .filter(|&i| !game.state.team_state[0].active_players[i].1.has_ball)
                            .collect();

                        if !players_without_ball.is_empty() {
                            let random_index = rng.gen_range(0..players_without_ball.len());
                            let selected_player_index = players_without_ball[random_index];
                            game.state.team_state[0].active_players[selected_player_index]
                                .1
                                .has_ball = true;
                        }
                        game.state.team_state[0].active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                    }
                } else if game.state.possession == Possession::Away {
                    let player_with_ball_index = game.state.team_state[1]
                        .active_players
                        .iter()
                        .position(|p| p.1.has_ball)
                        .unwrap();
                    let is_shot = game.state.team_state[1].active_players[player_with_ball_index]
                        .1
                        .is_shot();
                    if is_shot.0 {
                        let random = thread_rng().gen_range(0.0..1.0);
                        println!(
                            "RNG: {}, Shot Chance: {}",
                            random,
                            game.state.team_state[1].active_players[player_with_ball_index]
                                .1
                                .calculate_shot_chance(
                                    game.state.team_state[1].active_players[player_with_ball_index]
                                        .0
                                        .attributes()
                                )
                        );
                        if random
                            < game.state.team_state[1].active_players[player_with_ball_index]
                                .1
                                .calculate_shot_chance(
                                    game.state.team_state[1].active_players[player_with_ball_index]
                                        .0
                                        .attributes(),
                                )
                        {
                            // Shot made
                            println!(
                                "{} shoots from {:?} and makes it!",
                                game.state.team_state[1].active_players[player_with_ball_index]
                                    .0
                                    .first_name,
                                game.state.team_state[1].active_players[player_with_ball_index]
                                    .1
                                    .current_area
                            );
                            game.state.score.1 += is_shot.1;
                        } else {
                            println!(
                                "{} shoots from {:?} and misses it!",
                                game.state.team_state[1].active_players[player_with_ball_index]
                                    .0
                                    .first_name,
                                game.state.team_state[1].active_players[player_with_ball_index]
                                    .1
                                    .current_area
                            );
                        }
                        game.change_possession(Possession::Home);
                        let mut rng = thread_rng();
                        let player_index = rng.gen_range(0..5);
                        game.state.team_state[1].active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                        game.state.team_state[0].active_players[player_index]
                            .1
                            .has_ball = true;
                    }
                    if game.state.team_state[1].active_players[player_with_ball_index]
                        .1
                        .action
                        == PlayerAction::Pass
                    {
                        let mut rng = thread_rng();
                        let player_count = game.state.team_state[1].active_players.len();

                        // Collect indices of players without the ball
                        let players_without_ball: Vec<usize> = (0..player_count)
                            .filter(|&i| !game.state.team_state[1].active_players[i].1.has_ball)
                            .collect();

                        if !players_without_ball.is_empty() {
                            let random_index = rng.gen_range(0..players_without_ball.len());
                            let selected_player_index = players_without_ball[random_index];
                            game.state.team_state[1].active_players[selected_player_index]
                                .1
                                .has_ball = true;
                        }
                        game.state.team_state[1].active_players[player_with_ball_index]
                            .1
                            .has_ball = false;
                    }
                }
                //Generate random number between 1 and 24 float
                let mut rng = rand::thread_rng();
                let random = rng.gen_range(0.0..12.0);
                if random > game.state.time.as_secs_f32() {
                    game.state.time = Duration::from_secs(0);
                } else {
                    game.state.time = game.state.time - Duration::from_secs_f32(random);
                }
                //Generating new state for all players
                game.state
                    .team_state
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, s)| {
                        for (_, p) in s.active_players.iter_mut().enumerate() {
                            let is_offense: bool;
                            if game.state.possession == Possession::Home {
                                is_offense = i == 0;
                            } else {
                                is_offense = i == 1;
                            }
                            p.1.generate_next_player_state(
                                p.0.attributes(),
                                is_offense,
                                p.1.has_ball,
                            );
                        }
                    });

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
