use rusqlite::Connection;

pub mod event;
use crate::game::event::game_event;
use crate::player::player_state::PlayerState;
use crate::player::Player;
use crate::team::Team;

pub struct Game<'a> {
    teams: (&'a Team, &'a Team),
    score: (u32, u32),
    fouls: (u8, u8),
    timeouts: (u8, u8),
    events: Vec<game_event::GameEvent>,
    state: (Vec<PlayerState>, Vec<PlayerState>),
    players_in_play: (Vec<Player>, Vec<Player>),
    bench: (Vec<Player>, Vec<Player>),
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
            score: (0, 0),
            fouls: (0, 0),
            timeouts: (0, 0),
            events: Vec::new(),
            state: (Vec::new(), Vec::new()),
            players_in_play: starters,
            bench: bench,
        };
        Ok(game)
    }

    pub fn generate_next_game_event(&mut self) -> Result<(), String> {
        let _ = game_event::GameEvent::generate_next_game_event(self);

        println!("Team 0 State:");
        for (i, s) in self.state.0.iter().enumerate() {
            println!("Player {} State: {:?}", self.players_in_play.0[i], s);
        }
        println!("Team 1 State:");
        for (i, s) in self.state.1.iter().enumerate() {
            println!("Player {} State: {:?}", self.players_in_play.1[i], s);
        }
        Ok(())
    }
}
