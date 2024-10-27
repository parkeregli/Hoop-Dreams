use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CourtArea {
    Basket,
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

// Function to get the position weight for a court area
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

        // Out of bounds and boundaries
        CourtArea::SidelineLeft => f32::INFINITY,
        CourtArea::SidelineRight => f32::INFINITY,
        CourtArea::BaselineLeft => f32::INFINITY,
        CourtArea::BaselineRight => f32::INFINITY,
        CourtArea::OutOfBounds => f32::INFINITY,
    }
}

struct Node {
    area: CourtArea,
    f_score: f32,
    g_score: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_score.to_bits() == other.f_score.to_bits()
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Modified A* implementation that uses weights
pub fn find_path(start: CourtArea, goal: CourtArea) -> Option<Vec<CourtArea>> {
    // Handle same location case
    if start == goal {
        return Some(vec![start]);
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<CourtArea, CourtArea> = HashMap::new();
    let mut g_scores: HashMap<CourtArea, f32> = HashMap::new();
    let mut closed_set = HashSet::new();

    // Initialize start node
    g_scores.insert(start, 0.0);
    open_set.push(Node {
        area: start,
        f_score: get_position_weight(start),
        g_score: 0.0,
    });

    while let Some(current) = open_set.pop() {
        if current.area == goal {
            // Reconstruct path
            let mut path = vec![goal];
            let mut current_area = goal;

            while let Some(&prev) = came_from.get(&current_area) {
                path.push(prev);
                current_area = prev;
            }

            path.reverse();
            return Some(path);
        }

        if closed_set.contains(&current.area) {
            continue;
        }

        closed_set.insert(current.area);

        // Get neighboring areas
        for &next in can_move_to(current.area).iter() {
            if closed_set.contains(&next) {
                continue;
            }

            // Cost to move is the weight of the next position
            let tentative_g_score = current.g_score + get_position_weight(next);

            if tentative_g_score < *g_scores.get(&next).unwrap_or(&f32::INFINITY) {
                // This path is better than any previous one
                came_from.insert(next, current.area);
                g_scores.insert(next, tentative_g_score);

                // f_score is current cost plus estimated cost to goal
                let h_score = get_position_weight(next); // heuristic
                let f_score = tentative_g_score + h_score;

                open_set.push(Node {
                    area: next,
                    f_score,
                    g_score: tentative_g_score,
                });
            }
        }
    }

    None // No path found
}

#[test]
fn test_long_paths() {
    // Let's verify each path is actually possible by checking the graph connections
    let cases = vec![
        (CourtArea::Backcourt, CourtArea::RestrictedAreaMiddle, 4),
        // Path from left three point corner to right three point corner:
        // ThreePointLineCornerLeft -> ThreePointLineWingLeft ->
        // ThreePointLineCenter -> ThreePointLineWingRight -> ThreePointLineCornerRight
        (
            CourtArea::ThreePointLineCornerLeft,
            CourtArea::ThreePointLineCornerRight,
            4,
        ),
    ];

    for (start, goal, expected_moves) in cases {
        let result = find_path(start, goal);
        assert!(
            result.is_some(),
            "Failed to find path from {:?} to {:?}",
            start,
            goal
        );

        let path = result.unwrap();
        assert_eq!(
            path.len(),
            expected_moves,
            "Path length {} from {:?} to {:?} not equal to expected {}",
            path.len(),
            start,
            goal,
            expected_moves
        );
        assert_eq!(path[0], start);
        assert_eq!(*path.last().unwrap(), goal);
    }
}

impl CourtArea {
    pub fn shot_chance(&self) -> f32 {
        match self {
            CourtArea::Basket
            | CourtArea::RestrictedAreaLeft
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
    pub fn points(&self) -> u8 {
        let two_points = [
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
        let three_points = [
            CourtArea::ThreePointLineCornerRight,
            CourtArea::ThreePointLineCornerLeft,
            CourtArea::ThreePointLineWingRight,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineCenter,
            CourtArea::Backcourt,
            CourtArea::Center,
        ];
        if two_points.contains(self) {
            return 2;
        } else if three_points.contains(self) {
            return 3;
        } else {
            return 0;
        }
    }
    pub fn is_front_court(&self) -> bool {
        match self {
            CourtArea::Backcourt
            | CourtArea::OutOfBounds
            | CourtArea::SidelineLeft
            | CourtArea::SidelineRight
            | CourtArea::ThreePointLineCenter
            | CourtArea::ThreePointLineWingLeft
            | CourtArea::ThreePointLineWingRight => false,
            _ => true,
        }
    }
}

pub fn can_move_to(current_area: CourtArea) -> HashSet<CourtArea> {
    match current_area {
        CourtArea::Basket => [
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::RestrictedAreaLeft,
        ]
        .into(),
        CourtArea::RestrictedAreaLeft => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::LowPostLeft,
            CourtArea::ShortCornerLeft,
            CourtArea::ElbowLeft,
            CourtArea::FreeThrowLine,
            CourtArea::Basket,
        ]
        .into(),
        CourtArea::RestrictedAreaMiddle => [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::LowPostLeft,
            CourtArea::LowPostRight,
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
            CourtArea::Basket,
        ]
        .into(),
        CourtArea::RestrictedAreaRight => [
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::LowPostRight,
            CourtArea::ShortCornerRight,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
            CourtArea::Basket,
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
        ]
        .into(),
        CourtArea::ThreePointLineCornerRight => [
            CourtArea::ThreePointLineCornerRight,
            CourtArea::ThreePointLineWingRight,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeWingRight,
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

pub fn go_towards(area: CourtArea, target: CourtArea) -> CourtArea {
    if let Some(path) = find_path(area, target) {
        if path.len() == 1 {
            return path[0]; // Return the next step in the path
        }
        if path.len() >= 2 {
            return path[1]; // Return the next step in the path
        }
    }
    println!("No path found from {:?} to {:?}", area, target);
    area // Return current position if no path found
}

pub fn defend_towards(area: CourtArea, target: CourtArea) -> CourtArea {
    // Get path between target area and the basket
    if let Some(path) = find_path(target, CourtArea::Basket) {
        let mut area_selected = target;
        if path.len() == 1 {
            area_selected = path[0];
        } else if path.len() >= 2 {
            area_selected = path[1];
        }
        return area_selected;
    }
    area // Return current position if no path found
}

pub fn is_between_basket(area: CourtArea, target: CourtArea) -> bool {
    let path = find_path(target, CourtArea::Basket);
    if let Some(path) = path {
        path.contains(&area)
    } else {
        false
    }
}
