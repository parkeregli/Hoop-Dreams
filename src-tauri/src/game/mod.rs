use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub mod court;
pub mod event;
use crate::game::event::game_event;
use crate::player::player_state::PlayerState;
use crate::player::Player;
use crate::team::Team;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Possession {
    Home,
    Away,
    None,
}
struct GameState {
    time: Duration,
    period: u8,
    possession: Possession,
    players_in_play: ([Player; 5], [Player; 5]),
    player_states: ([PlayerState; 5], [PlayerState; 5]),
    bench: (Vec<Player>, Vec<Player>),
    fouls: (u8, u8),
    timeouts: (u8, u8),
    score: (u32, u32),
}
pub struct Game<'a> {
    teams: (&'a Team, &'a Team),
    events: Vec<game_event::GameEvent>,
    state: GameState,
}

impl Game<'_> {
    pub fn new<'a>(
        db: &Connection,
        home_team: &'a Team,
        away_team: &'a Team,
    ) -> Result<Game<'a>, rusqlite::Error> {
        let starters = (
            home_team
                .get_starting_lineup(db)
                .expect("Error getting starting lineups"),
            away_team
                .get_starting_lineup(db)
                .expect("Error getting starting lineups"),
        );
        let bench = (
            home_team.get_bench(db).expect("Error getting bench."),
            away_team.get_bench(db).expect("Error getting bench."),
        );
        let game = Game {
            teams: (home_team, away_team),
            state: GameState {
                period: 1,
                possession: Possession::None,
                score: (0, 0),
                fouls: (0, 0),
                timeouts: (0, 0),
                players_in_play: starters.clone(),
                player_states: (
                    [(); 5].map(|_| PlayerState::new(false, false, None)),
                    [(); 5].map(|_| PlayerState::new(false, false, None)),
                ),

                //720 = 12 minutes
                time: Duration::from_secs(720),
                bench,
            },
            events: Vec::new(),
        };
        Ok(game)
    }

    pub fn generate_next_game_event(&mut self) -> Result<(), String> {
        for _ in 0..10 {
            let _ = game_event::GameEvent::generate_next_game_event(self);

            println!("Team 0 State:");
            for (i, s) in self.state.player_states.0.iter().enumerate() {
                println!(
                    "Player: {} {} State: {:?}",
                    self.state.players_in_play.0[i].first_name,
                    self.state.players_in_play.0[i].last_name,
                    s
                );
            }
            println!("Team 1 State:");
            for (i, s) in self.state.player_states.1.iter().enumerate() {
                println!(
                    "Player {} {} State: {:?}",
                    self.state.players_in_play.1[i].first_name,
                    self.state.players_in_play.1[i].last_name,
                    s
                );
            }
        }
        Ok(())
    }
}
