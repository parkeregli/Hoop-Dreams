// court.rs

// Dimensions of the basketball court in feet
const COURT_LENGTH: f32 = 94.0;
const COURT_WIDTH: f32 = 50.0;
const THREE_POINT_LINE_DISTANCE: f32 = 23.75;
const THREE_POINT_LINE_CORNER_DISTANCE: f32 = 22.0;
const FREE_THROW_LINE_DISTANCE: f32 = 15.0;
const KEY_WIDTH: f32 = 16.0;
const KEY_HEIGHT: f32 = 19.0;
const HOOP_HEIGHT: f32 = 10.0;
const GRID_SIZE: f32 = 2.0;

pub struct Court {
    pub length: f32,
    pub width: f32,
    pub three_point_line_distance: f32,
    pub three_point_line_corner_distance: f32,
    pub free_throw_line_distance: f32,
    pub key_width: f32,
    pub key_height: f32,
    pub hoop_height: f32,
    pub grid_size: f32,
}

impl Court {
    pub fn new() -> Court {
        Court {
            length: COURT_LENGTH,
            width: COURT_WIDTH,
            three_point_line_distance: THREE_POINT_LINE_DISTANCE,
            three_point_line_corner_distance: THREE_POINT_LINE_CORNER_DISTANCE,
            free_throw_line_distance: FREE_THROW_LINE_DISTANCE,
            key_width: KEY_WIDTH,
            key_height: KEY_HEIGHT,
            hoop_height: HOOP_HEIGHT,
            grid_size: GRID_SIZE,
        }
    }

    pub fn is_inside_court(&self, x: i32, y: i32) -> bool {
        let x_pos = (x as f32) * self.grid_size;
        let y_pos = (y as f32) * self.grid_size;
        x_pos.abs() <= self.length / 2.0 && y_pos.abs() <= self.width / 2.0
    }

    pub fn is_inside_key(&self, x: i32, y: i32) -> bool {
        let x_pos = (x as f32) * self.grid_size;
        let y_pos = (y as f32) * self.grid_size;
        x_pos.abs() <= self.key_height / 2.0 && y_pos.abs() <= self.key_width / 2.0
    }

    pub fn is_three_point_shot(&self, x: i32, y: i32) -> bool {
      let x_pos = (x as f32) * self.grid_size;
      let y_pos = (y as f32) * self.grid_size;
      let distance_from_basket = ((x_pos - self.length / 2.0).powi(2) + y_pos.powi(2)).sqrt();

      let corner_length = 14.0; // Length of the corner area in feet
      let corner_width = 3.0; // Width of the corner area in feet

      if x_pos >= self.length / 2.0 - corner_length && y_pos.abs() >= self.width / 2.0 - corner_width {
          // Shot is from the corner area
          distance_from_basket >= self.three_point_line_corner_distance
      } else {
          // Regular three-point shot
          distance_from_basket >= self.three_point_line_distance
      }
    }
}
