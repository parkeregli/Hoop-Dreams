mod court_graph;
mod court_zones;

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

// Re-export public functions from submodules
pub use court_graph::can_move_to;
pub use court_zones::get_position_weight;

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

// A* pathfinding implementation

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
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the optimal path between two court areas using A* algorithm.
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

/// Returns the next area to move to when going towards a target.
pub fn go_towards(area: CourtArea, target: CourtArea) -> CourtArea {
    if let Some(path) = find_path(area, target) {
        if path.len() == 1 {
            return path[0];
        }
        if path.len() >= 2 {
            return path[1];
        }
    }
    println!("No path found from {:?} to {:?}", area, target);
    area
}

/// Returns the area a defender should move to when guarding a target.
pub fn defend_towards(area: CourtArea, target: CourtArea) -> CourtArea {
    if let Some(path) = find_path(target, CourtArea::Basket) {
        let mut area_selected = target;
        if path.len() == 1 {
            area_selected = path[0];
        } else if path.len() >= 2 {
            area_selected = path[1];
        }
        return area_selected;
    }
    area
}

/// Returns true if the given area is on the path between target and basket.
pub fn is_between_basket(area: CourtArea, target: CourtArea) -> bool {
    find_path(target, CourtArea::Basket)
        .map(|path| path.contains(&area))
        .unwrap_or(false)
}

#[test]
fn test_long_paths() {
    let cases = vec![
        (CourtArea::Backcourt, CourtArea::RestrictedAreaMiddle, 4),
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
