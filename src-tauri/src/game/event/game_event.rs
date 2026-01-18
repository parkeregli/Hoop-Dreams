use crate::game::game_config::{overtime_duration, period_duration, NUM_PERIODS};
use crate::game::possession::{switch_possession, Possession};
use crate::game::Game;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Maximum time decrement per event during regular play.
const MAX_TIME_DECREMENT_REGULAR: f32 = 3.0;
/// Maximum time decrement per event during overtime.
const MAX_TIME_DECREMENT_OVERTIME: f32 = 6.0;

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
        // Time still on the clock
        if game.state.time.as_secs() > 0 {
            return Self::handle_active_play(game, MAX_TIME_DECREMENT_REGULAR);
        }

        // End of regulation with tied score - go to overtime
        if game.state.period >= NUM_PERIODS && game.state.score.0 == game.state.score.1 {
            game.state.period += 1;
            game.state.time = overtime_duration();
            return Self::handle_active_play(game, MAX_TIME_DECREMENT_OVERTIME);
        }

        // End of regulation with winner
        if game.state.period >= NUM_PERIODS {
            game.state.time = Duration::from_secs(0);
            return Ok(GameEvent::new(
                "End of Game".to_string(),
                game.get_time(),
                0,
                None,
            ));
        }

        // End of quarter
        game.state.period += 1;
        game.state.time = period_duration();
        Ok(GameEvent::new(
            "End of Quarter".to_string(),
            game.get_time(),
            0,
            None,
        ))
    }

    /// Handles active play including shot clock management and time decrement.
    fn handle_active_play(game: &mut Game, max_decrement: f32) -> Result<GameEvent, String> {
        let event = game.handle_player_actions()?;

        let mut rng = rand::thread_rng();
        let max = f32::min(max_decrement, game.state.time.as_secs_f32());
        let time_elapsed = rng.gen_range(1.0..max);

        // Check for shot clock violation
        if time_elapsed > game.state.shot_clock.as_secs_f32() {
            println!("Shot clock ran out. Turnover!");
            Self::handle_shot_clock_violation(game);
            return Ok(GameEvent::new(
                "Turnover".to_string(),
                game.get_time(),
                game.state.period,
                None,
            ));
        }

        // Update clocks
        game.state.shot_clock -= Duration::from_secs_f32(time_elapsed);
        game.state.time -= Duration::from_secs_f32(time_elapsed);

        Ok(event)
    }

    /// Handles a shot clock violation by switching possession.
    fn handle_shot_clock_violation(game: &mut Game) {
        const TURNOVER_PLAYER_INDEX: usize = 3;
        let new_possession = switch_possession(&game.state.possession, TURNOVER_PLAYER_INDEX);
        game.change_possession(new_possession);
    }
}
