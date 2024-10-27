use crate::game::court::CourtArea;
use rand::Rng;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use super::Player;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
    pub spd: i32,
    pub interior_def: i32,
    pub perimeter_def: i32,
    pub close_shot: i32,
    pub mid_shot: i32,
    pub deep_shot: i32,
    pub shot_in_traffic: i32,
    pub intelligence: i32,
    pub handle: i32,
    pub pass: i32,
    pub block: i32,
    pub steal: i32,
    pub off_rebound: i32,
    pub def_rebound: i32,
    pub ath: i32,
    pub strength: i32,
    pub durability: i32,
    pub conditioning: i32,
}

impl PlayerAttributes {
    pub fn new() -> PlayerAttributes {
        PlayerAttributes {
            spd: 0,
            interior_def: 0,
            perimeter_def: 0,
            close_shot: 0,
            mid_shot: 0,
            deep_shot: 0,
            shot_in_traffic: 0,
            intelligence: 0,
            handle: 0,
            pass: 0,
            block: 0,
            steal: 0,
            off_rebound: 0,
            def_rebound: 0,
            ath: 0,
            strength: 0,
            durability: 0,
            conditioning: 0,
        }
    }
    pub fn shot_chance(&self, area: CourtArea) -> f32 {
        match area {
            CourtArea::RestrictedAreaLeft
            | CourtArea::RestrictedAreaRight
            | CourtArea::RestrictedAreaMiddle
            | CourtArea::Basket
            | CourtArea::LowPostLeft
            | CourtArea::LowPostRight
            | CourtArea::ShortCornerLeft
            | CourtArea::ShortCornerRight => {
                return self.close_shot as f32;
            }
            CourtArea::ElbowLeft
            | CourtArea::ElbowRight
            | CourtArea::FreeThrowLine
            | CourtArea::MidrangeCenter
            | CourtArea::MidrangeWingLeft
            | CourtArea::MidrangeWingRight
            | CourtArea::MidrangeBaselineLeft
            | CourtArea::MidrangeBaselineRight => {
                return self.mid_shot as f32;
            }
            CourtArea::ThreePointLineCornerLeft
            | CourtArea::ThreePointLineCornerRight
            | CourtArea::ThreePointLineWingLeft
            | CourtArea::ThreePointLineWingRight
            | CourtArea::ThreePointLineCenter => {
                return self.deep_shot as f32;
            }
            CourtArea::Center => {
                return 0.1;
            }
            CourtArea::Backcourt
            | CourtArea::SidelineLeft
            | CourtArea::SidelineRight
            | CourtArea::BaselineLeft
            | CourtArea::BaselineRight
            | CourtArea::OutOfBounds => {
                return 0.0;
            }
        }
    }

    pub fn get_player_attributes(
        player: &Player,
        db: &Connection,
    ) -> Result<PlayerAttributes, rusqlite::Error> {
        let mut stmt = db.prepare(
            "SELECT
                spd,
                interior_def,
                perimeter_def,
                close_shot,
                mid_shot,
                deep_shot,
                shot_in_traffic,
                intelligence,
                handle,
                pass,
                block,
                steal,
                off_rebound,
                def_rebound,
                athleticism,
                strength,
                durability,
                conditioning
            FROM
                player_attributes
            WHERE
                player_id = ?",
        )?;
        let player_attributes = stmt.query_row([player.get_id().unwrap()], |row| {
            Ok(PlayerAttributes {
                spd: row.get(0)?,
                interior_def: row.get(1)?,
                perimeter_def: row.get(2)?,
                close_shot: row.get(3)?,
                mid_shot: row.get(4)?,
                deep_shot: row.get(5)?,
                shot_in_traffic: row.get(6)?,
                intelligence: row.get(7)?,
                handle: row.get(8)?,
                pass: row.get(9)?,
                block: row.get(10)?,
                steal: row.get(11)?,
                off_rebound: row.get(12)?,
                def_rebound: row.get(13)?,
                ath: row.get(14)?,
                strength: row.get(15)?,
                durability: row.get(16)?,
                conditioning: row.get(17)?,
            })
        })?;

        Ok(player_attributes)
    }
}
pub fn gen_rand_attrs() -> PlayerAttributes {
    let mut rand = rand::thread_rng();

    PlayerAttributes {
        spd: rand.gen_range(0..100),
        interior_def: rand.gen_range(0..100),
        perimeter_def: rand.gen_range(0..100),
        close_shot: rand.gen_range(0..100),
        mid_shot: rand.gen_range(0..100),
        deep_shot: rand.gen_range(0..100),
        shot_in_traffic: rand.gen_range(0..100),
        intelligence: rand.gen_range(0..100),
        handle: rand.gen_range(0..100),
        pass: rand.gen_range(0..100),
        block: rand.gen_range(0..100),
        steal: rand.gen_range(0..100),
        off_rebound: rand.gen_range(0..100),
        def_rebound: rand.gen_range(0..100),
        ath: rand.gen_range(0..100),
        strength: rand.gen_range(0..100),
        durability: rand.gen_range(0..100),
        conditioning: rand.gen_range(0..100),
    }
}
