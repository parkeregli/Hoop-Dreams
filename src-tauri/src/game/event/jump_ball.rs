use crate::game::game_event::GameEvent;
use crate::game::Game;
use crate::player::player_attributes;
use crate::player::player_state::{PlayerAction, PlayerState};
use crate::util::rng::rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::Connection;

use std::collections::HashMap;
use std::time::Duration;

pub fn generate_jump_ball(game: &mut Game, db: &Connection) -> Result<(), String> {
    // Jump Ball
    let starters = (
        game.teams
            .0
            .get_starting_lineup(db)
            .expect("Error getting starting lineups"),
        game.teams
            .1
            .get_starting_lineup(db)
            .expect("Error getting starting lineups"),
    );
    game.set_players_in_play(starters);

    let mut best_jmp = HashMap::new();

    best_jmp.insert("Home", (None, 0));
    best_jmp.insert("Away", (None, 0));
    //Array for the two best jumpers by reference

    //Return the best jumper and the ratio
    //Map starting lineup with jmp_ball_ratio
    for player in &game.players_in_play.0 {
        //Get player with best jmp + height
        let player_attributes = player_attributes::gen_rand_attrs();
        let jmp_ball_ratio = player_attributes.ath + player.get_height();

        if jmp_ball_ratio > best_jmp.get("Home").unwrap().1 {
            best_jmp.insert("Home", (Some(player), jmp_ball_ratio));
        }
    }

    for player in &game.players_in_play.1 {
        //Get player with best jmp + height
        let player_attributes = player_attributes::gen_rand_attrs();
        let jmp_ball_ratio = player_attributes.ath + player.get_height();

        if jmp_ball_ratio > best_jmp.get("Away").unwrap().1 {
            best_jmp.insert("Away", (Some(player), jmp_ball_ratio));
        }
    }

    let winner = rng(
        best_jmp.get("Home").unwrap().1,
        best_jmp.get("Away").unwrap().1,
    );

    println!("Winner: {}", winner);
    let mut rng = thread_rng();
    if winner == 0 {
        //Home
        //Using clone for now. It is not performant.
        let has_ball = game.players_in_play.0.choose(&mut rng).unwrap();
        //Find index of has_ball in players_in_play by id
        let mut team_state: (Vec<PlayerState>, Vec<PlayerState>) = (vec![], vec![]);
        game.players_in_play.0.iter().for_each(|p| {
            println!(
                "{:?}, {:?}",
                p.get_id().unwrap(),
                has_ball.get_id().unwrap()
            );
            if p.get_id().unwrap() == has_ball.get_id().unwrap() {
                team_state.0.push(PlayerState::new(true, true));
                team_state.1.push(PlayerState::new(false, true));
            } else {
                team_state.0.push(PlayerState::new(true, false));
                team_state.1.push(PlayerState::new(false, false));
            }
        });
        game.state = team_state;
        let event = GameEvent::new(
            has_ball.clone(),
            format!(
                "Jump Ball won for {} by {} {}. {} {} has the ball.",
                "Home",
                best_jmp.get("Home").unwrap().0.unwrap().first_name,
                best_jmp.get("Home").unwrap().0.unwrap().last_name,
                &has_ball.first_name,
                &has_ball.last_name
            ),
            Duration::from_secs(60 * 12),
            1,
            Some("Home".to_string()),
        );
        println!("Event: {:?}", event.action);
        game.events.push(event);
    } else {
        let has_ball = game.players_in_play.1.choose(&mut rng).unwrap();

        let mut team_state: (Vec<PlayerState>, Vec<PlayerState>) = (vec![], vec![]);
        game.players_in_play.1.iter().for_each(|p| {
            println!(
                "{:?}, {:?}",
                p.get_id().unwrap(),
                has_ball.get_id().unwrap()
            );
            if p.get_id().unwrap() == has_ball.get_id().unwrap() {
                team_state.1.push(PlayerState::new(true, true));
                team_state.0.push(PlayerState::new(false, true));
            } else {
                team_state.1.push(PlayerState::new(true, false));
                team_state.0.push(PlayerState::new(false, false));
            }
        });
        game.state = team_state;
        let event = GameEvent::new(
            has_ball.clone(),
            format!(
                "Jump Ball won for {} by {} {}. {} {} has the ball.",
                "Away",
                best_jmp.get("Away").unwrap().0.unwrap().first_name,
                best_jmp.get("Away").unwrap().0.unwrap().last_name,
                &has_ball.first_name,
                &has_ball.last_name
            ),
            Duration::from_secs(60 * 12),
            1,
            Some("Away".to_string()),
        );
        println!("Event: {:?}", event.action);
        game.events.push(event);
    }

    println!("Team 0 State:");
    game.state.0.iter().for_each(|p| {
        println!("{:?}", p);
    });
    println!("Team 1 State:");
    game.state.1.iter().for_each(|p| {
        println!("{:?}", p);
    });
    return Ok(());
}
