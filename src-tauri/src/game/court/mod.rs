use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CourtArea {
    // Inside the three-point line
    RestrictedAreaLeft,
    RestrictedAreaMiddle,
    RestrictedAreaRight,
    LowPostLeft,
    LowPostRight,
    ShortCornerLeft,
    ShortCornerRight,
    ElbowLeft,
    ElbowRight,
    FreeThrowLine,

    // Midrange areas
    MidrangeBaselineLeft,
    MidrangeBaselineRight,
    MidrangeWingLeft,
    MidrangeCenter,
    MidrangeWingRight,

    // Three-point line and beyond
    ThreePointLineCornerLeft,
    ThreePointLineCornerRight,
    ThreePointLineWingLeft,
    ThreePointLineWingRight,
    ThreePointLineCenter,

    // Other areas
    Center,
    Backcourt,

    // Out of bounds areas
    SidelineLeft,
    SidelineRight,
    BaselineLeft,
    BaselineRight,
    OutOfBounds,
}

impl CourtArea {
    pub fn shot_chance(&self) -> f32 {
        match self {
            CourtArea::RestrictedAreaLeft
            | CourtArea::RestrictedAreaRight
            | CourtArea::RestrictedAreaMiddle => 1.0,
            CourtArea::LowPostRight | CourtArea::LowPostLeft => 0.9,
            CourtArea::ShortCornerLeft | CourtArea::ShortCornerRight => 0.85,
            CourtArea::ElbowLeft | CourtArea::ElbowRight | CourtArea::FreeThrowLine => 0.8,
            CourtArea::Center => 0.1,
            CourtArea::ThreePointLineCornerLeft | CourtArea::ThreePointLineCornerRight => 1.0,
            CourtArea::ThreePointLineWingLeft
            | CourtArea::ThreePointLineWingRight
            | CourtArea::ThreePointLineCenter => 1.0,
            CourtArea::MidrangeCenter
            | CourtArea::MidrangeWingLeft
            | CourtArea::MidrangeWingRight => 0.7,
            CourtArea::MidrangeBaselineLeft | CourtArea::MidrangeBaselineRight => 0.7,
            CourtArea::Backcourt => 0.0,
            CourtArea::SidelineLeft
            | CourtArea::SidelineRight
            | CourtArea::BaselineLeft
            | CourtArea::BaselineRight
            | CourtArea::OutOfBounds => 0.0,
        }
    }
    pub fn is_front_court(&self) -> bool {
        match self {
            CourtArea::Backcourt
            | CourtArea::OutOfBounds
            | CourtArea::SidelineLeft
            | CourtArea::SidelineRight
            | CourtArea::BaselineLeft
            | CourtArea::BaselineRight => false,
            _ => true,
        }
    }
}

pub fn can_move_to(current_area: CourtArea) -> HashSet<CourtArea> {
    match current_area {
        CourtArea::RestrictedAreaLeft => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::LowPostLeft,
            CourtArea::ShortCornerLeft,
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
        ]
        .into(),
        CourtArea::RestrictedAreaMiddle => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::LowPostLeft,
            CourtArea::LowPostRight,
            CourtArea::ShortCornerLeft,
            CourtArea::ShortCornerRight,
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
        ]
        .into(),
        CourtArea::RestrictedAreaRight => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::LowPostRight,
            CourtArea::ShortCornerRight,
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
        ]
        .into(),
        CourtArea::LowPostLeft => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::LowPostLeft,
            CourtArea::ShortCornerLeft,
            CourtArea::ElbowLeft,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::MidrangeWingLeft,
        ]
        .into(),
        CourtArea::LowPostRight => [
            CourtArea::RestrictedAreaRight,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::LowPostRight,
            CourtArea::ShortCornerRight,
            CourtArea::ElbowRight,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeWingRight,
        ]
        .into(),
        CourtArea::ShortCornerLeft => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::ShortCornerLeft,
            CourtArea::ElbowLeft,
            CourtArea::MidrangeBaselineLeft,
        ]
        .into(),
        CourtArea::ShortCornerRight => [
            CourtArea::RestrictedAreaRight,
            CourtArea::ShortCornerRight,
            CourtArea::ElbowRight,
            CourtArea::MidrangeBaselineRight,
        ]
        .into(),
        CourtArea::ElbowLeft => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::ElbowLeft,
            CourtArea::FreeThrowLine,
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::MidrangeCenter,
        ]
        .into(),
        CourtArea::ElbowRight => [
            CourtArea::RestrictedAreaRight,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeCenter,
        ]
        .into(),
        CourtArea::FreeThrowLine => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::FreeThrowLine,
            CourtArea::LowPostLeft,
            CourtArea::LowPostRight,
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeCenter,
        ]
        .into(),
        CourtArea::MidrangeBaselineLeft => [
            CourtArea::MidrangeBaselineLeft,
            CourtArea::ShortCornerLeft,
            CourtArea::LowPostLeft,
            CourtArea::ElbowLeft,
            CourtArea::MidrangeWingLeft,
            CourtArea::ThreePointLineCornerLeft,
        ]
        .into(),
        CourtArea::MidrangeBaselineRight => [
            CourtArea::MidrangeBaselineRight,
            CourtArea::ShortCornerRight,
            CourtArea::LowPostRight,
            CourtArea::ElbowRight,
            CourtArea::MidrangeWingRight,
            CourtArea::ThreePointLineCornerRight,
        ]
        .into(),
        CourtArea::MidrangeWingLeft => [
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeCenter,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::FreeThrowLine,
            CourtArea::ElbowLeft,
            CourtArea::LowPostLeft,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineCenter,
        ]
        .into(),
        CourtArea::MidrangeWingRight => [
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeCenter,
            CourtArea::MidrangeBaselineRight,
            CourtArea::FreeThrowLine,
            CourtArea::ElbowRight,
            CourtArea::LowPostRight,
            CourtArea::ThreePointLineWingRight,
            CourtArea::ThreePointLineCenter,
        ]
        .into(),
        CourtArea::MidrangeCenter => [
            CourtArea::MidrangeCenter,
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeWingRight,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineWingRight,
            CourtArea::ThreePointLineCenter,
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
        ]
        .into(),
        CourtArea::ThreePointLineCornerLeft => [
            CourtArea::ThreePointLineCornerLeft,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::MidrangeWingLeft,
            CourtArea::ShortCornerLeft,
        ]
        .into(),
        CourtArea::ThreePointLineCornerRight => [
            CourtArea::ThreePointLineCornerRight,
            CourtArea::ThreePointLineWingRight,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeWingRight,
            CourtArea::ShortCornerRight,
        ]
        .into(),
        CourtArea::ThreePointLineWingLeft => [
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineCenter,
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeCenter,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::ThreePointLineCornerLeft,
        ]
        .into(),
        CourtArea::ThreePointLineWingRight => [
            CourtArea::ThreePointLineWingRight,
            CourtArea::ThreePointLineCenter,
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeCenter,
            CourtArea::MidrangeBaselineRight,
            CourtArea::ThreePointLineCornerRight,
        ]
        .into(),
        CourtArea::ThreePointLineCenter => [
            CourtArea::ThreePointLineCenter,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineWingRight,
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeCenter,
            CourtArea::Center,
        ]
        .into(),
        CourtArea::Center => [
            CourtArea::Center,
            CourtArea::ThreePointLineCenter,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineWingRight,
            CourtArea::Backcourt,
        ]
        .into(),
        CourtArea::Backcourt => [
            CourtArea::Backcourt,
            CourtArea::Center,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineWingRight,
        ]
        .into(),
        CourtArea::SidelineLeft => [
            CourtArea::SidelineLeft,
            CourtArea::ThreePointLineCornerLeft,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeBaselineLeft,
        ]
        .into(),
        CourtArea::SidelineRight => [
            CourtArea::SidelineRight,
            CourtArea::ThreePointLineCornerRight,
            CourtArea::ThreePointLineWingRight,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeWingRight,
        ]
        .into(),
        CourtArea::BaselineLeft => [
            CourtArea::BaselineLeft,
            CourtArea::LowPostLeft,
            CourtArea::ShortCornerLeft,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
        ]
        .into(),
        CourtArea::BaselineRight => [
            CourtArea::BaselineRight,
            CourtArea::LowPostRight,
            CourtArea::ShortCornerRight,
            CourtArea::MidrangeBaselineRight,
            CourtArea::RestrictedAreaRight,
            CourtArea::RestrictedAreaMiddle,
        ]
        .into(),
        CourtArea::OutOfBounds => [CourtArea::OutOfBounds].into(),
    }
}
