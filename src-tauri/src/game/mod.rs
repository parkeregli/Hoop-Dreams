use event::game_event::GameEvent;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub mod court;
pub mod event;
use crate::game::court::CourtArea;
use crate::game::event::game_event;
use crate::game::event::jump_ball;
use crate::player::player_state::PlayerAction;
use crate::player::player_state::PlayerState;
use crate::player::Player;
use crate::team::Team;
use rand::{thread_rng, Rng};
use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Possession {
    Home,
    Away,
}
impl fmt::Display for Possession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Possession::Home => write!(f, "Home"),
            Possession::Away => write!(f, "Away"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TeamState {
    active_players: [(Player, PlayerState); 5],
    bench: (Vec<Player>, Vec<Player>),
}
impl TeamState {
    pub fn new(starters: [Player; 5], bench: Vec<Player>) -> Self {
        Self {
            active_players: starters.map(|p| (p, PlayerState::new(false, None))),
            bench: (Vec::new(), Vec::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    time: Duration,
    shot_clock: Duration,
    period: u8,
    possession: Option<(Possession, usize)>,
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
        let teams = Team::get_teams_from_db(db);
        match teams {
            Ok(teams) => {
                let home_players = (
                    teams[0]
                        .get_starting_lineup(db)
                        .expect("Error getting starting lineups"),
                    teams[0].get_bench(db).expect("Error getting bench."),
                );
                let away_players = (
                    teams[1]
                        .get_starting_lineup(db)
                        .expect("Error getting starting lineups"),
                    teams[1].get_bench(db).expect("Error getting bench."),
                );
                let home_state = TeamState::new(home_players.0, home_players.1);
                let away_state = TeamState::new(away_players.0, away_players.1);
                let game = Game {
                    teams: (teams[0].clone(), teams[1].clone()),
                    state: GameState {
                        period: 1,
                        shot_clock: Duration::from_secs(24),
                        possession: None,
                        score: (0, 0),
                        fouls: (0, 0),
                        timeouts: (0, 0),
                        team_state: [home_state, away_state],
                        //720 = 12 minutes
                        time: Duration::from_secs(720),
                    },
                    events: Vec::new(),
                    sim: false,
                };
                Ok(game)
            }
            Err(e) => Err(e),
        }
    }

    pub fn change_possession(&mut self, new_possession: Option<(Possession, usize)>) {
        let possession_changed = match (&self.state.possession, &new_possession) {
            (None, None) => false,
            (Some((old_possession, _)), Some((new_possession, _))) => {
                old_possession != new_possession
            }
            _ => true,
        };
        if possession_changed {
            self.state
                .team_state
                .iter_mut()
                .enumerate()
                .for_each(|(_, s)| {
                    s.active_players.iter_mut().enumerate().for_each(|(_, p)| {
                        if p.1.current_area.is_front_court() {
                            p.1.current_area = CourtArea::Backcourt;
                        }
                        if p.1.current_area == CourtArea::Backcourt {
                            p.1.current_area = CourtArea::Center;
                        }
                    })
                });
            self.state.shot_clock = Duration::from_secs(24);
        }
        self.state.possession = new_possession;
    }

    pub fn player_has_ball(&self) -> Option<(&Player, &PlayerState)> {
        match self.state.possession {
            Some((Possession::Home, index)) => {
                let player = &self.state.team_state[0].active_players[index];
                Some((&player.0, &player.1))
            }
            Some((Possession::Away, index)) => {
                let player = &self.state.team_state[1].active_players[index];
                Some((&player.0, &player.1))
            }
            None => None,
        }
    }

    pub fn handle_player_actions(&mut self) -> Result<GameEvent, String> {
        let mut message = String::new();
        let mut event: Option<GameEvent> = None;
        let mut new_possession: Option<(Possession, usize)> = self.state.possession;
        let mut points_added: u8 = 0;
        if let Some((player, player_state)) = self.player_has_ball() {
            let buzzer_beater = self.state.shot_clock < Duration::from_millis(500)
                || self.state.time < Duration::from_millis(500);
            if buzzer_beater || player_state.is_shot().is_some() {
                let points: u8 = if buzzer_beater {
                    player_state.current_area.points()
                } else {
                    player_state.is_shot().expect("No points generated")
                };
                let random = thread_rng().gen_range(0.0..1.0);
                let shot_chance = player_state.calculate_shot_chance(player.attributes());
                println!("RNG: {}, Shot Chance: {}", random, shot_chance);
                if shot_chance > random {
                    //Shot made
                    message = format!(
                        "{} {} {:?} from {:?} and makes it!",
                        player.first_name,
                        player.last_name,
                        player_state.action,
                        player_state.current_area
                    );

                    points_added = points;
                } else {
                    message = format!(
                        "{} {} {:?} from {:?} and misses it!",
                        player.first_name,
                        player.last_name,
                        player_state.action,
                        player_state.current_area
                    );
                }
                let mut rng = thread_rng();
                let player_index = rng.gen_range(0..5);
                match self.state.possession {
                    Some((Possession::Home, _)) => {
                        new_possession = Some((Possession::Away, player_index));
                    }
                    Some((Possession::Away, _)) => {
                        new_possession = Some((Possession::Home, player_index));
                    }
                    None => {}
                }
            }
            if player_state.action == PlayerAction::Pass {
                let mut rng = thread_rng();
                let mut random_index = rng.gen_range(0..5);
                match self.state.possession {
                    Some((Possession::Home, index)) => {
                        while index == random_index {
                            random_index = rng.gen_range(0..5);
                        }
                        new_possession = Some((Possession::Home, random_index));
                        let has_ball =
                            self.state.team_state[0].active_players[random_index].clone();
                        message = format!(
                            "{} {} passes to {} {} from {:?} to {:?}",
                            player.first_name,
                            player.last_name,
                            has_ball.0.first_name,
                            has_ball.0.last_name,
                            player_state.current_area,
                            has_ball.1.current_area,
                        )
                    }
                    Some((Possession::Away, index)) => {
                        while index == random_index {
                            random_index = rng.gen_range(0..5);
                        }
                        new_possession = Some((Possession::Away, random_index));
                        let has_ball =
                            self.state.team_state[1].active_players[random_index].clone();
                        message = format!(
                            "{} {} passes to {} {} from {:?} to {:?}",
                            player.first_name,
                            player.last_name,
                            has_ball.0.first_name,
                            has_ball.0.last_name,
                            player_state.current_area,
                            has_ball.1.current_area,
                        )
                    }
                    None => {}
                }
            }
            if player_state.action == PlayerAction::Drive {
                message = format!(
                    "{} {} drives to {:?}",
                    player.first_name, player.last_name, player_state.current_area,
                );
            }
            match self.state.possession {
                Some((Possession::Home, _)) => {
                    //Convert time to mm:ss:ms
                    event = Some(GameEvent::new(
                        message,
                        self.get_time(),
                        self.state.period,
                        Some(Possession::Home),
                    ));
                }
                Some((Possession::Away, _)) => {
                    event = Some(GameEvent::new(
                        message,
                        self.get_time(),
                        self.state.period,
                        Some(Possession::Away),
                    ));
                }

                None => {
                    event = Some(GameEvent::new(
                        message,
                        self.get_time(),
                        self.state.period,
                        None,
                    ));
                }
            }
        }

        match self.state.possession {
            Some((Possession::Home, _)) => {
                self.state.score.0 += points_added;
            }
            Some((Possession::Away, _)) => {
                self.state.score.1 += points_added;
            }
            None => {}
        }
        self.change_possession(new_possession);
        self.update_player_states();

        if event.is_none() {
            return Err("No event generated".to_string());
        } else {
            Ok(event.unwrap())
        }
    }
    pub fn get_time(&self) -> String {
        let minutes = self.state.time.as_secs() / 60;
        let seconds = self.state.time.as_secs() % 60;
        let milliseconds = self.state.time.as_millis() % 1000;
        let time = format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds);
        time
    }
    pub fn get_score(&self) -> (u8, u8) {
        (self.state.score.0, self.state.score.1)
    }
    pub fn update_player_states(&mut self) {
        self.state
            .team_state
            .iter_mut()
            .enumerate()
            .for_each(|(i, s)| {
                s.active_players.iter_mut().enumerate().for_each(|(j, p)| {
                    match self.state.possession {
                        Some((Possession::Home, index)) => {
                            if i == 0 {
                                if j == index {
                                    p.1.generate_next_player_state(p.0.attributes(), true, true);
                                } else {
                                    p.1.generate_next_player_state(p.0.attributes(), true, false)
                                }
                            } else {
                                p.1.generate_next_player_state(p.0.attributes(), false, false)
                            }
                        }
                        Some((Possession::Away, index)) => {
                            if i == 1 {
                                if j == index {
                                    p.1.generate_next_player_state(p.0.attributes(), true, true);
                                } else {
                                    p.1.generate_next_player_state(p.0.attributes(), true, false)
                                }
                            } else {
                                p.1.generate_next_player_state(p.0.attributes(), false, false)
                            }
                        }
                        None => {
                            p.1.generate_next_player_state(p.0.attributes(), false, false)
                        }
                    }
                })
            });
    }

    pub fn generate_next_game_event(&mut self) -> Result<GameEvent, String> {
        if self.events.len() == 0 {
            let _ = jump_ball::generate_jump_ball(self);
            self.state.time = Duration::from_secs(720);
        }
        //Print score
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
        self.state.team_state.iter().enumerate().for_each(|(i, s)| {
            println!("Team: {}", if i == 0 { "Home" } else { "Away" },);
            for (_, p) in s.active_players.iter().enumerate() {
                println!(
                    "Player: {} {} State: {:?}",
                    p.0.first_name, p.0.last_name, p.1
                );
            }
        });
        let event = game_event::GameEvent::generate_next_game_event(self);
        println!("------------------------------------------------------");
        //Wait 3 seconds
        std::thread::sleep(std::time::Duration::from_millis(250));
        Ok(event.unwrap())
    }
}
