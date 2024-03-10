use crate::player::Player;
use std::fmt;

#[derive(Debug)]
pub struct Team<'a> {
    name: &'a str,
    city: &'a str,
    starting_lineup: [Option<Player<'a>>; 5],
    bench: [Option<Player<'a>>; 8],
}

impl<'a> Team<'a> {
    pub fn new(name: &'a str, city: &'a str) -> Team<'a> {
        Team {
            name,
            city,
            starting_lineup: [None; 5],
            bench: [None; 8],
        }
    }

    pub fn add_player_to_starting_lineup(&mut self, player: Player<'a>, position: usize) {
        if position < 5 {
            self.starting_lineup[position] = Some(player);
        } else {
            println!("Invalid starting lineup position");
        }
    }

    pub fn add_player_to_bench(&mut self, player: Player<'a>, position: usize) {
        if position < 8 {
            self.bench[position] = Some(player);
        } else {
            println!("Invalid bench position");
        }
    }

    pub fn remove_player_from_starting_lineup(&mut self, position: usize) {
        if position < 5 {
            self.starting_lineup[position] = None;
        } else {
            println!("Invalid starting lineup position");
        }
    }

    pub fn remove_player_from_bench(&mut self, position: usize) {
        if position < 8 {
            self.bench[position] = None;
        } else {
            println!("Invalid bench position");
        }
    }

    pub fn get_starting_lineup(&self) -> &[Option<Player>; 5] {
        &self.starting_lineup
    }

    pub fn get_bench(&self) -> &[Option<Player>; 8] {
        &self.bench
    }
}

impl fmt::Display for Team<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Team: {}, City: {}", self.name, self.city)?;
        writeln!(f, "Starting Lineup:")?;
        for (i, player) in self.starting_lineup.iter().enumerate() {
            match player {
                Some(p) => writeln!(f, "Position {}: {}", i + 1, p)?,
                None => writeln!(f, "Position {}: Empty", i + 1)?,
            }
        }
        writeln!(f, "Bench:")?;
        for (i, player) in self.bench.iter().enumerate() {
            match player {
                Some(p) => writeln!(f, "Bench {}: {}", i + 1, p)?,
                None => writeln!(f, "Bench {}: Empty", i + 1)?,
            }
        }
        Ok(())
    }
}
