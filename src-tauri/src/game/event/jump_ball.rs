use crate::game::game_event::GameEvent;
use crate::game::{Game, Possession};
use crate::util::rng::rng;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub fn generate_jump_ball(game: &mut Game) -> Result<(), String> {
    // Collect the necessary data without holding references
    let home_best = game
        .state
        .team_state
        .0
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

    let away_best = game
        .state
        .team_state
        .1
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

    let (possession, team_state, best_jumper) = if winner == 0 {
        (Possession::Home, &mut game.state.team_state.0, home_best)
    } else {
        (Possession::Away, &mut game.state.team_state.1, away_best)
    };

    game.state.possession = possession;
    let player_index = rng.gen_range(0..team_state.active_players.len());
    team_state.active_players[player_index].1.has_ball = true;

    let has_ball = &team_state.active_players[player_index].0;

    let event = GameEvent::new(
        format!(
            "Jump Ball won for {} by {} {}. {} {} has the ball.",
            if winner == 0 { "Home" } else { "Away" },
            best_jumper.1,
            best_jumper.2,
            &has_ball.first_name,
            &has_ball.last_name
        ),
        game.state.time - Duration::from_secs(1),
        1,
        possession,
    );

    println!("Event: {:?}", event.action);
    game.events.push(event);

    Ok(())
}
