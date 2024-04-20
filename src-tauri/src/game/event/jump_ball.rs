use crate::game::game_event::GameEvent;
use crate::game::Game;
use crate::player::player_attributes;
use rusqlite::Connection;

use std::collections::HashMap;
use std::time::Duration;

pub fn generate_jump_ball(game: &mut Game, db: &Connection) -> Result<(), String> {
    // Jump Ball
    let (home_starting_lineup, away_starting_lineup) = (
        game.teams
            .0
            .get_starting_lineup(db)
            .expect("Error getting starting lineups"),
        game.teams
            .1
            .get_starting_lineup(db)
            .expect("Error getting starting lineups"),
    );

    let mut best_jmp = HashMap::new();

    best_jmp.insert("Home", (None, 0));
    best_jmp.insert("Away", (None, 0));
    //Array for the two best jumpers by reference

    //Return the best jumper and the ratio
    for player in &home_starting_lineup {
        //Get player with best jmp + height
        let player_attributes = player_attributes::gen_rand_attrs();
        let jmp_ball_ratio = player_attributes.ath + player.get_height();

        if jmp_ball_ratio > best_jmp.get("Home").unwrap().1 {
            best_jmp.insert("Home", (Some(player), jmp_ball_ratio));
        }
    }

    for player in &away_starting_lineup {
        //Get player with best jmp + height
        let player_attributes = player_attributes::gen_rand_attrs();
        let jmp_ball_ratio = player_attributes.ath + player.get_height();

        if jmp_ball_ratio > best_jmp.get("Away").unwrap().1 {
            best_jmp.insert("Away", (Some(player), jmp_ball_ratio));
        }
    }

    let winner = GameEvent::rng(
        best_jmp.get("Home").unwrap().1,
        best_jmp.get("Away").unwrap().1,
    );

    println!("Winner: {}", winner);
    if winner == 0 {
        let event = GameEvent::new(
            format!(
                "Jump Ball won for {} by {} {}",
                "Home",
                best_jmp.get("Home").unwrap().0.unwrap().first_name,
                best_jmp.get("Home").unwrap().0.unwrap().last_name
            ),
            Duration::from_secs(60 * 12),
            1,
            Some("Home".to_string()),
            (home_starting_lineup, away_starting_lineup),
        );
        println!("Event: {:?}", event.action);
        game.events.push(event);
    } else {
        let event = GameEvent::new(
            format!(
                "Jump Ball won for {} by {} {}",
                "Away",
                best_jmp.get("Away").unwrap().0.unwrap().first_name,
                best_jmp.get("Away").unwrap().0.unwrap().last_name
            ),
            Duration::from_secs(60 * 12),
            1,
            Some("Away".to_string()),
            (home_starting_lineup, away_starting_lineup),
        );
        println!("Event: {:?}", event.action);
        game.events.push(event);
    }

    return Ok(());
}
