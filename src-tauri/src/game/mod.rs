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
}

impl Game<'_> {
    pub fn new<'a>(home_team: &'a Team, away_team: &'a Team) -> Result<Game<'a>, rusqlite::Error> {
        Ok(Game {
            teams: (home_team, away_team),
            score: (0, 0),
            fouls: (0, 0),
            timeouts: (0, 0),
            events: Vec::new(),
            state: (Vec::new(), Vec::new()),
            players_in_play: (Vec::new(), Vec::new()),
        })
    }

    pub fn generate_next_game_event(&mut self, db: &Connection) -> Result<(), String> {
        let _ = game_event::GameEvent::generate_next_game_event(self, db)?;
        Ok(())
    }

    pub fn set_players_in_play(&mut self, players_in_play: (Vec<Player>, Vec<Player>)) {
        self.players_in_play = players_in_play;
    }
}
