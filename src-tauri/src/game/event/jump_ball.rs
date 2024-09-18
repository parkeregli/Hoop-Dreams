use crate::game::game_event::GameEvent;
use crate::game::{Game, Possession};
use crate::util::rng::rng;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::time::Duration;

pub fn generate_jump_ball(game: &mut Game) -> Result<(), String> {
    let mut best_jmp = HashMap::new();

    best_jmp.insert("Home", (None, 0));
    best_jmp.insert("Away", (None, 0));
    //Array for the two best jumpers by reference

    //Return the best jumper and the ratio
    //Map starting lineup with jmp_ball_ratio
    for player in &game.state.players_in_play.0 {
        //Get player with best jmp + height
        let jmp_ball_ratio = player.attributes().ath + player.get_height();

        if jmp_ball_ratio > best_jmp.get("Home").unwrap().1 {
            best_jmp.insert("Home", (Some(player), jmp_ball_ratio));
        }
    }

    for player in &game.state.players_in_play.1 {
        //Get player with best jmp + height
        let jmp_ball_ratio = player.attributes().ath + player.get_height();

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
        game.state.possession = Possession::Home;
        //Get random index of home team players
        let home_idx = rng.gen_range(0..5);
        let has_ball = &game.state.players_in_play.0[home_idx];

        for (i, p) in game.state.player_states.0.iter_mut().enumerate() {
            p.generate_next_player_state(i == home_idx, true);
        }
        for (_, p) in game.state.player_states.1.iter_mut().enumerate() {
            p.generate_next_player_state(false, false);
        }

        let event = GameEvent::new(
            format!(
                "Jump Ball won for {} by {} {}. {} {} has the ball.",
                "Home",
                best_jmp.get("Home").unwrap().0.unwrap().first_name,
                best_jmp.get("Home").unwrap().0.unwrap().last_name,
                &has_ball.first_name,
                &has_ball.last_name
            ),
            game.state.time - Duration::from_secs(1),
            1,
            Possession::Home,
        );
        println!("Event: {:?}", event.action);
        game.events.push(event);
    } else {
        game.state.possession = Possession::Away;
        //Get random index of home team players
        let away_idx = rng.gen_range(0..5);
        let has_ball = &game.state.players_in_play.1[away_idx];

        for (_, p) in game.state.player_states.0.iter_mut().enumerate() {
            p.generate_next_player_state(false, false);
        }
        for (i, p) in game.state.player_states.1.iter_mut().enumerate() {
            p.generate_next_player_state(i == away_idx, true);
        }
        let event = GameEvent::new(
            format!(
                "Jump Ball won for {} by {} {}. {} {} has the ball.",
                "Away",
                best_jmp.get("Away").unwrap().0.unwrap().first_name,
                best_jmp.get("Away").unwrap().0.unwrap().last_name,
                &has_ball.first_name,
                &has_ball.last_name
            ),
            game.state.time - Duration::from_secs(1),
            1,
            Possession::Away,
        );
        println!("Event: {:?}", event.action);
        game.events.push(event);
    }
    return Ok(());
}
