use super::CourtArea;

/// Position weights for pathfinding - lower values indicate positions closer to the basket.
pub fn get_position_weight(area: CourtArea) -> f32 {
    match area {
        // Closest to basket - highest reward/lowest cost
        CourtArea::Basket => 0.0,
        CourtArea::RestrictedAreaMiddle => 1.0,
        CourtArea::RestrictedAreaLeft => 1.2,
        CourtArea::RestrictedAreaRight => 1.2,

        // Next closest positions
        CourtArea::LowPostLeft => 1.5,
        CourtArea::LowPostRight => 1.5,
        CourtArea::ShortCornerLeft => 1.7,
        CourtArea::ShortCornerRight => 1.7,

        // Mid-range positions
        CourtArea::ElbowLeft => 2.0,
        CourtArea::ElbowRight => 2.0,
        CourtArea::FreeThrowLine => 2.2,
        CourtArea::MidrangeBaselineLeft => 2.5,
        CourtArea::MidrangeBaselineRight => 2.5,
        CourtArea::MidrangeWingLeft => 2.3,
        CourtArea::MidrangeWingRight => 2.3,
        CourtArea::MidrangeCenter => 2.7,

        // Three point line positions
        CourtArea::ThreePointLineCornerLeft => 3.0,
        CourtArea::ThreePointLineCornerRight => 3.0,
        CourtArea::ThreePointLineWingLeft => 3.2,
        CourtArea::ThreePointLineWingRight => 3.2,
        CourtArea::ThreePointLineCenter => 3.5,

        // Furthest positions
        CourtArea::Center => 4.0,
        CourtArea::Backcourt => 5.0,

        // Out of bounds and boundaries - infinite cost
        CourtArea::SidelineLeft
        | CourtArea::SidelineRight
        | CourtArea::BaselineLeft
        | CourtArea::BaselineRight
        | CourtArea::OutOfBounds => f32::INFINITY,
    }
}

impl CourtArea {
    /// Returns the shot success modifier for this court area (0.0 - 1.0).
    pub fn shot_chance(&self) -> f32 {
        match self {
            // Restricted area - highest chance
            CourtArea::Basket
            | CourtArea::RestrictedAreaLeft
            | CourtArea::RestrictedAreaRight
            | CourtArea::RestrictedAreaMiddle => 1.0,

            // Low post - very good chance
            CourtArea::LowPostRight | CourtArea::LowPostLeft => 0.9,

            // Short corner - good chance
            CourtArea::ShortCornerLeft | CourtArea::ShortCornerRight => 0.85,

            // Elbow/free throw - decent chance
            CourtArea::ElbowLeft | CourtArea::ElbowRight | CourtArea::FreeThrowLine => 0.8,

            // Mid-range - moderate chance
            CourtArea::MidrangeCenter
            | CourtArea::MidrangeWingLeft
            | CourtArea::MidrangeWingRight
            | CourtArea::MidrangeBaselineLeft
            | CourtArea::MidrangeBaselineRight => 0.7,

            // Three-point line - full difficulty (modified by player skill)
            CourtArea::ThreePointLineCornerLeft
            | CourtArea::ThreePointLineCornerRight
            | CourtArea::ThreePointLineWingLeft
            | CourtArea::ThreePointLineWingRight
            | CourtArea::ThreePointLineCenter => 1.0,

            // Center court - very low chance
            CourtArea::Center => 0.1,

            // Backcourt and out of bounds - cannot shoot
            CourtArea::Backcourt
            | CourtArea::SidelineLeft
            | CourtArea::SidelineRight
            | CourtArea::BaselineLeft
            | CourtArea::BaselineRight
            | CourtArea::OutOfBounds => 0.0,
        }
    }

    /// Returns the point value for shots from this court area.
    pub fn points(&self) -> u8 {
        const TWO_POINT_AREAS: &[CourtArea] = &[
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::LowPostLeft,
            CourtArea::LowPostRight,
            CourtArea::FreeThrowLine,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::RestrictedAreaLeft,
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeWingLeft,
            CourtArea::ShortCornerRight,
            CourtArea::ShortCornerLeft,
            CourtArea::MidrangeCenter,
        ];

        const THREE_POINT_AREAS: &[CourtArea] = &[
            CourtArea::ThreePointLineCornerRight,
            CourtArea::ThreePointLineCornerLeft,
            CourtArea::ThreePointLineWingRight,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineCenter,
            CourtArea::Backcourt,
            CourtArea::Center,
        ];

        if TWO_POINT_AREAS.contains(self) {
            2
        } else if THREE_POINT_AREAS.contains(self) {
            3
        } else {
            0
        }
    }

    /// Returns true if this area is in the front court (offensive half).
    pub fn is_front_court(&self) -> bool {
        !matches!(
            self,
            CourtArea::Backcourt
                | CourtArea::OutOfBounds
                | CourtArea::SidelineLeft
                | CourtArea::SidelineRight
                | CourtArea::ThreePointLineCenter
                | CourtArea::ThreePointLineWingLeft
                | CourtArea::ThreePointLineWingRight
        )
    }
}
