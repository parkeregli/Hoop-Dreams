use std::fmt;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    position: String,
    age: u8,
    height: u8,
    weight: u16,
    overall_rating: u8,
    shooting: u8,
    dribbling: u8,
    passing: u8,
    defense: u8,
    rebounding: u8,
}

impl Player {
    pub fn new(
        name: String,
        position: String,
        age: u8,
        height: u8,
        weight: u16,
        overall_rating: u8,
        shooting: u8,
        dribbling: u8,
        passing: u8,
        defense: u8,
        rebounding: u8,
    ) -> Player {
        Player {
            name,
            position,
            age,
            height,
            weight,
            overall_rating,
            shooting,
            dribbling,
            passing,
            defense,
            rebounding,
        }
    }

    pub fn update_rating(&mut self, new_rating: u8) {
        self.overall_rating = new_rating;
    }

    pub fn improve_skill(&mut self, skill: &str, amount: u8) {
        match skill {
            "shooting" => self.shooting = (self.shooting + amount).min(100),
            "dribbling" => self.dribbling = (self.dribbling + amount).min(100),
            "passing" => self.passing = (self.passing + amount).min(100),
            "defense" => self.defense = (self.defense + amount).min(100),
            "rebounding" => self.rebounding = (self.rebounding + amount).min(100),
            _ => println!("Invalid skill"),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}, Position: {}, Age: {}, Height: {}, Weight: {}, Overall Rating: {}, Shooting: {}, Dribbling: {}, Passing: {}, Defense: {}, Rebounding: {}",
            self.name, self.position, self.age, self.height, self.weight, self.overall_rating, self.shooting, self.dribbling, self.passing, self.defense, self.rebounding
        )
    }
}
