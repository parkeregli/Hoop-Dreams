use crate::game::game_event::GameEvent;
use crate::game::{Game, Possession};
use crate::util::rng::rng;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub fn generate_jump_ball(game: &mut Game) -> Result<(), String> {
    // Collect the necessary data without holding references
    let home_best = game.state.team_state[0]
        .active_players
        .iter()
        .map(|p| {
            (
                p.0.attributes().ath + p.0.get_height(),
                p.0.first_name.clone(),
                p.0.last_name.clone(),
            )
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .ok_or("No home players")?;

    let away_best = game.state.team_state[1]
        .active_players
        .iter()
        .map(|p| {
            (
                p.0.attributes().ath + p.0.get_height(),
                p.0.first_name.clone(),
                p.0.last_name.clone(),
            )
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .ok_or("No away players")?;

    let winner = rng(home_best.0, away_best.0);
    println!("Winner: {}", winner);

    let mut rng = thread_rng();

    let (possession, winning_team_state, best_jumper) = if winner == 0 {
        (Possession::Home, &mut game.state.team_state[0], home_best)
    } else {
        (Possession::Away, &mut game.state.team_state[1], away_best)
    };

    let player_index = rng.gen_range(0..winning_team_state.active_players.len());
    game.change_possession(Some((possession, player_index)));

    let has_ball = game.player_has_ball();
    match has_ball {
        Some((player, _)) => {
            let event = GameEvent::new(
                format!(
                    "Jump Ball won for {} by {} {}. {} {} has the ball.",
                    if winner == 0 { "Home" } else { "Away" },
                    best_jumper.1,
                    best_jumper.2,
                    player.first_name,
                    player.last_name
                ),
                game.state.time - Duration::from_secs(1),
                1,
                Some(possession),
            );
            game.events.push(event);
        }
        None => {
            return Ok(());
        }
    }
    game.update_player_states();
    Ok(())
}
