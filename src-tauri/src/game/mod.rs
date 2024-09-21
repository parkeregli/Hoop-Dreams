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
struct TeamState {
    active_players: [(Player, PlayerState); 5],
    bench: (Vec<Player>, Vec<Player>),
}
impl TeamState {
    pub fn new(starters: [Player; 5], bench: Vec<Player>) -> Self {
        Self {
            active_players: starters.map(|p| (p, PlayerState::new(false, false, None))),
            bench: (Vec::new(), Vec::new()),
        }
    }
}
struct GameState {
    time: Duration,
    period: u8,
    possession: Possession,
    team_state: (TeamState, TeamState),
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
        let home_players = (
            home_team
                .get_starting_lineup(db)
                .expect("Error getting starting lineups"),
            home_team.get_bench(db).expect("Error getting bench."),
        );
        let away_players = (
            away_team
                .get_starting_lineup(db)
                .expect("Error getting starting lineups"),
            away_team.get_bench(db).expect("Error getting bench."),
        );
        let home_state = TeamState::new(home_players.0, home_players.1);
        let away_state = TeamState::new(away_players.0, away_players.1);
        let game = Game {
            teams: (home_team, away_team),
            state: GameState {
                period: 1,
                possession: Possession::None,
                score: (0, 0),
                fouls: (0, 0),
                timeouts: (0, 0),
                team_state: (home_state, away_state),
                //720 = 12 minutes
                time: Duration::from_secs(720),
            },
            events: Vec::new(),
        };
        Ok(game)
    }

    pub fn generate_next_game_event(&mut self) -> Result<(), String> {
        for _ in 0..10 {
            //Print score
            println!("Possession: {:?}", self.state.possession);

            println!("Team 0 State:");
            for (i, s) in self.state.team_state.0.active_players.iter().enumerate() {
                println!(
                    "Player: {} {} State: {:?}",
                    self.state.team_state.0.active_players[i].0.first_name,
                    self.state.team_state.0.active_players[i].0.last_name,
                    self.state.team_state.0.active_players[i].1
                );
            }
            println!("Team 1 State:");
            for (i, s) in self.state.team_state.0.active_players.iter().enumerate() {
                println!(
                    "Player: {} {} State: {:?}",
                    self.state.team_state.1.active_players[i].0.first_name,
                    self.state.team_state.1.active_players[i].0.last_name,
                    self.state.team_state.1.active_players[i].1
                );
            }
            let _ = game_event::GameEvent::generate_next_game_event(self);
            println!("Home: {}, Away: {}", self.state.score.0, self.state.score.1);
            println!("------------------------------------------------------");
        }
        Ok(())
    }
}
