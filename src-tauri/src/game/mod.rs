use event::game_event::GameEvent;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub mod court;
pub mod event;
pub mod game_config;
pub mod possession;
pub mod rebound;

use crate::game::court::CourtArea;
use crate::game::event::game_event;
use crate::game::event::jump_ball;
use crate::game::game_config::*;
use crate::player::player_state::PlayerAction;
use crate::player::player_state::PlayerState;
use crate::player::Player;
use crate::team::Team;
use rand::{thread_rng, Rng};
use std::time::Duration;

// Re-export types for external use
pub use possession::{BallPossession, Possession};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TeamState {
    active_players: [(Player, PlayerState); 5],
    bench: (Vec<Player>, Vec<Player>),
}

impl TeamState {
    pub fn new(starters: [Player; 5], bench: Vec<Player>) -> Self {
        Self {
            active_players: starters.map(|p| (p, PlayerState::new(None))),
            bench: (bench, Vec::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    time: Duration,
    shot_clock: Duration,
    period: u8,
    possession: BallPossession,
    team_state: [TeamState; 2],
    fouls: (u8, u8),
    timeouts: (u8, u8),
    score: (u8, u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    teams: (Team, Team),
    events: Vec<game_event::GameEvent>,
    state: GameState,
    sim: bool,
}

impl Game {
    pub fn new(db: &Connection) -> Result<Game, rusqlite::Error> {
        let teams = Team::get_teams_from_db(db)?;
        if teams.len() < 2 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let home_starting = teams[0].get_starting_lineup(db)?;
        let home_bench = teams[0].get_bench(db)?;
        let away_starting = teams[1].get_starting_lineup(db)?;
        let away_bench = teams[1].get_bench(db)?;

        let home_state = TeamState::new(home_starting, home_bench);
        let away_state = TeamState::new(away_starting, away_bench);

        let game = Game {
            teams: (teams[0].clone(), teams[1].clone()),
            state: GameState {
                period: 1,
                shot_clock: shot_clock_duration(),
                possession: None,
                score: (0, 0),
                fouls: (0, 0),
                timeouts: (0, 0),
                team_state: [home_state, away_state],
                time: period_duration(),
            },
            events: Vec::new(),
            sim: false,
        };
        Ok(game)
    }

    pub fn change_possession(&mut self, new_possession: BallPossession) {
        self.change_possession_with_clock(new_possession, None);
    }

    pub fn change_possession_with_clock(
        &mut self,
        new_possession: BallPossession,
        shot_clock_override: Option<Duration>,
    ) {
        let possession_changed = self.has_possession_changed(&new_possession);

        if possession_changed {
            self.reset_player_positions();
        }

        if let Some(d) = shot_clock_override {
            self.state.shot_clock = d;
        } else if possession_changed {
            self.state.shot_clock = shot_clock_duration();
        }

        self.state.possession = new_possession;
    }

    fn has_possession_changed(&self, new_possession: &BallPossession) -> bool {
        match (&self.state.possession, new_possession) {
            (None, None) => false,
            (Some((old, _)), Some((new, _))) => old != new,
            _ => true,
        }
    }

    fn reset_player_positions(&mut self) {
        for team_state in self.state.team_state.iter_mut() {
            for (_, player_state) in team_state.active_players.iter_mut() {
                player_state.current_area = if player_state.current_area.is_front_court() {
                    CourtArea::Backcourt
                } else {
                    CourtArea::Center
                };
            }
        }
    }

    pub fn player_has_ball(&self) -> Option<(&Player, &PlayerState)> {
        let (possession, player_index) = self.state.possession?;
        let team_index = possession.team_index();
        let player = &self.state.team_state[team_index].active_players[player_index];
        Some((&player.0, &player.1))
    }

    pub fn handle_player_actions(&mut self) -> Result<GameEvent, String> {
        let mut message = String::new();
        let mut new_possession = self.state.possession;
        let mut points_added: u8 = 0;
        let mut possession_handled = false;

        if let Some((player, player_state)) = self.player_has_ball() {
            let is_buzzer_beater = self.state.shot_clock < buzzer_beater_threshold()
                || self.state.time < buzzer_beater_threshold();

            // Handle shot attempts
            if is_buzzer_beater || player_state.is_shot().is_some() {
                let (msg, points, possession, shot_clock) =
                    self.handle_shot(player, player_state, is_buzzer_beater);
                message = msg;
                points_added = points;
                new_possession = possession;
                self.change_possession_with_clock(new_possession, shot_clock);
                let _ = self.update_player_states();
                possession_handled = true;
            } else {
                // Handle passes
                if player_state.action == PlayerAction::Pass {
                    let (msg, possession) = self.handle_pass(player, player_state);
                    message = msg;
                    new_possession = possession;
                }

                // Handle drives
                if player_state.action == PlayerAction::Drive {
                    message = format!(
                        "{} {} drives to {:?}",
                        player.first_name, player.last_name, player_state.current_area,
                    );
                }
            }
        }

        // Update score
        self.add_points(points_added);

        // Create event
        let possession_for_event = self.state.possession.map(|(p, _)| p);
        let event = GameEvent::new(
            message,
            self.get_time(),
            self.state.period,
            possession_for_event,
        );

        if !possession_handled {
            self.change_possession(new_possession);
            let _ = self.update_player_states();
        }

        Ok(event)
    }

    fn handle_shot(
        &self,
        player: &Player,
        player_state: &PlayerState,
        is_buzzer_beater: bool,
    ) -> (String, u8, BallPossession, Option<Duration>) {
        let Some((possession, ball_holder_idx)) = self.state.possession else {
            return (String::new(), 0, self.state.possession, None);
        };

        let offensive_team = possession.team_index();
        let defensive_team = 1 - offensive_team;

        let defender = &self.state.team_state[defensive_team].active_players[ball_holder_idx];
        let defender_state = &defender.1;
        let defender_attributes = defender.0.attributes();

        let defender_in_range = player_state
            .current_area
            .is_adjacent_to(defender_state.current_area)
            || player_state.current_area == defender_state.current_area;

        let contest_str = if defender_in_range {
            let stance = match defender_state.action {
                PlayerAction::DefendTight => "tightly defended by",
                PlayerAction::Defend => "defended by",
                PlayerAction::DefendLoose => "loosely defended by",
                _ => "contested by",
            };
            format!(
                " {} {} {}",
                stance, defender.0.first_name, defender.0.last_name
            )
        } else {
            String::new()
        };

        let points = if is_buzzer_beater {
            player_state.current_area.points()
        } else {
            player_state
                .is_shot()
                .unwrap_or_else(|| player_state.current_area.points())
        };

        let random: f32 = thread_rng().gen_range(0.0..1.0);
        let shot_chance = player_state.calculate_shot_chance_with_defender(
            player.attributes(),
            Some(defender_state),
            Some(defender_attributes),
        );
        println!("RNG: {}, Shot Chance: {}", random, shot_chance);

        let block_random: f32 = thread_rng().gen_range(0.0..1.0);
        let block_chance =
            defender_state.calculate_block_chance(defender_attributes, player_state.current_area);

        let (mut message, points_scored) = if block_chance > block_random {
            println!(
                "BLOCK! Block chance: {}, RNG: {}",
                block_chance, block_random
            );
            (
                format!(
                    "{} {} {:?} from {:?} and gets BLOCKED by {} {}!",
                    player.first_name,
                    player.last_name,
                    player_state.action,
                    player_state.current_area,
                    defender.0.first_name,
                    defender.0.last_name
                ),
                0,
            )
        } else if shot_chance > random {
            (
                format!(
                    "{} {} {:?} from {:?} and makes it!{}",
                    player.first_name,
                    player.last_name,
                    player_state.action,
                    player_state.current_area,
                    contest_str
                ),
                points,
            )
        } else {
            (
                format!(
                    "{} {} {:?} from {:?} and misses it!{}",
                    player.first_name,
                    player.last_name,
                    player_state.action,
                    player_state.current_area,
                    contest_str
                ),
                0,
            )
        };

        let (new_possession, shot_clock) = if points_scored > 0 {
            let new_player_index = thread_rng().gen_range(0..PLAYERS_PER_TEAM);
            (
                possession::switch_possession(&self.state.possession, new_player_index),
                None,
            )
        } else {
            let rebound_zone = rebound::pick_rebound_zone(player_state.current_area);
            let contenders = rebound::get_players_near_rebound_zone(self, rebound_zone);
            let winner = rebound::resolve_rebound(&contenders, rebound_zone);

            if let Some(winner) = winner {
                let is_offensive = rebound::is_offensive_rebound(&winner, &self.state.possession);

                if points_scored == 0 {
                    let winner_player = &self.state.team_state[winner.team_index].active_players
                        [winner.player_index]
                        .0;

                    let rebound_message = if is_offensive {
                        format!(
                            "{} {} grabs the offensive rebound!",
                            winner_player.first_name, winner_player.last_name
                        )
                    } else {
                        format!(
                            "{} {} grabs the defensive rebound!",
                            winner_player.first_name, winner_player.last_name
                        )
                    };

                    message = format!("{} {}", message, rebound_message);
                }

                let new_possession: BallPossession = Some((
                    if winner.team_index == 0 {
                        Possession::Home
                    } else {
                        Possession::Away
                    },
                    winner.player_index,
                ));

                let shot_clock = if is_offensive {
                    // If shot clock is below 15, reset to 15
                    if self.state.shot_clock < Duration::from_secs(15) {
                        Some(Duration::from_secs(15))
                    } else {
                        None
                    }
                } else {
                    None
                };

                (new_possession, shot_clock)
            } else {
                let new_player_index = thread_rng().gen_range(0..PLAYERS_PER_TEAM);
                let new_possession =
                    possession::switch_possession(&self.state.possession, new_player_index);
                (new_possession, None)
            }
        };

        (message, points_scored, new_possession, shot_clock)
    }

    fn handle_pass(&self, player: &Player, player_state: &PlayerState) -> (String, BallPossession) {
        let Some((possession, current_index)) = self.state.possession else {
            return (String::new(), self.state.possession);
        };

        let team_index = possession.team_index();
        let mut rng = thread_rng();
        let mut target_index = rng.gen_range(0..PLAYERS_PER_TEAM);

        // Make sure we don't pass to ourselves
        while target_index == current_index {
            target_index = rng.gen_range(0..PLAYERS_PER_TEAM);
        }

        let receiver = &self.state.team_state[team_index].active_players[target_index];
        let message = format!(
            "{} {} passes to {} {} from {:?} to {:?}",
            player.first_name,
            player.last_name,
            receiver.0.first_name,
            receiver.0.last_name,
            player_state.current_area,
            receiver.1.current_area,
        );

        let new_possession = Some((possession, target_index));
        (message, new_possession)
    }

    fn add_points(&mut self, points: u8) {
        if points == 0 {
            return;
        }

        if let Some((possession, _)) = self.state.possession {
            match possession {
                Possession::Home => self.state.score.0 += points,
                Possession::Away => self.state.score.1 += points,
            }
        }
    }

    pub fn get_time(&self) -> String {
        let minutes = self.state.time.as_secs() / 60;
        let seconds = self.state.time.as_secs() % 60;
        let milliseconds = self.state.time.as_millis() % 1000;
        format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
    }

    pub fn get_score(&self) -> (u8, u8) {
        (self.state.score.0, self.state.score.1)
    }

    pub fn get_clock(&self) -> (Duration, Duration, u8, BallPossession) {
        (
            self.state.time,
            self.state.shot_clock,
            self.state.period,
            self.state.possession,
        )
    }

    pub fn get_player_states(&self) -> Vec<(Player, PlayerState)> {
        self.state.team_state[0]
            .active_players
            .iter()
            .chain(self.state.team_state[1].active_players.iter())
            .cloned()
            .collect()
    }

    pub fn update_player_states(&mut self) -> Result<(), String> {
        let Some((possession, ball_holder_index)) = self.state.possession else {
            return Ok(());
        };

        let offensive_team = possession.team_index();
        let defensive_team = 1 - offensive_team;

        // Clone offensive player states so defenders can track their assignments
        // Each defender at index i guards the offensive player at index i (man-to-man)
        let offensive_states: Vec<PlayerState> = self.state.team_state[offensive_team]
            .active_players
            .iter()
            .map(|(_, state)| state.clone())
            .collect();

        // Update offensive players
        for (i, (player, state)) in self.state.team_state[offensive_team]
            .active_players
            .iter_mut()
            .enumerate()
        {
            let has_ball = i == ball_holder_index;
            let _ = state.generate_next_player_state(
                player.attributes(),
                (true, has_ball),
                (false, None),
            );
        }

        // Update defensive players - each defender tracks their offensive assignment
        for (i, (player, state)) in self.state.team_state[defensive_team]
            .active_players
            .iter_mut()
            .enumerate()
        {
            let _ = state.generate_next_player_state(
                player.attributes(),
                (false, false),
                (true, Some(&offensive_states[i])),
            );
        }

        Ok(())
    }

    pub fn generate_next_game_event(&mut self) -> Result<GameEvent, String> {
        if self.events.is_empty() {
            let _ = jump_ball::generate_jump_ball(self);
            self.state.time = period_duration();
        }

        self.print_debug_info();

        let event = game_event::GameEvent::generate_next_game_event(self)?;
        println!("------------------------------------------------------");
        Ok(event)
    }

    fn print_debug_info(&self) {
        println!("------------------------------------------------------");
        println!("Home: {}, Away: {}", self.state.score.0, self.state.score.1);

        let total_ms = self.state.time.as_millis();
        let minutes = total_ms / 60000;
        let seconds = (total_ms % 60000) / 1000;
        let milliseconds = (total_ms % 1000) / 10;
        let sc_seconds = self.state.shot_clock.as_secs();
        let sc_milliseconds = self.state.shot_clock.as_millis() % 1000;

        println!(
            "Period: {} | Time: {:02}:{:02}:{:03} | Shotclock: {:02}:{:03}",
            self.state.period, minutes, seconds, milliseconds, sc_seconds, sc_milliseconds
        );
        println!("Possession: {:?}", self.state.possession);

        for (i, team_state) in self.state.team_state.iter().enumerate() {
            println!("Team: {}", if i == 0 { "Home" } else { "Away" });
            for (player, state) in team_state.active_players.iter() {
                println!(
                    "Player: {} {} State: {:?}",
                    player.first_name, player.last_name, state
                );
            }
        }
    }

    pub fn jump_to_target(&mut self, target: JumpTarget) -> Result<(), String> {
        while self.state.period < target.target_period() {
            while self.state.time.as_secs() > 0 {
                let _ = self.generate_next_game_event()?;
            }
            if self.state.period >= NUM_PERIODS && self.state.score.0 == self.state.score.1 {
                self.state.period += 1;
                self.state.time = overtime_duration();
            } else if self.state.period >= NUM_PERIODS {
                break;
            } else {
                self.state.period += 1;
                self.state.time = period_duration();
            }
        }

        if target == JumpTarget::GameEnd {
            while self.state.time.as_secs() > 0 {
                let event = self.generate_next_game_event()?;
                if event.is_game_end() {
                    break;
                }
            }
        } else {
            self.state.time = target.target_time();
            self.state.shot_clock = shot_clock_duration();
        }

        Ok(())
    }
}
