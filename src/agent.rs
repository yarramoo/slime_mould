use rand::Rng;
use slime_mould::{random_orientation, DataMap, Position, Radians};

use crate::cells::trail_cell::TrailCell;

const SENSOR_ANGLE: f32 = 22.5;
const ROTATION_ANGLE: f32 = 45.;
const SENSOR_OFFSET_DISTANCE: f32 = 9.;
const SENSOR_WIDTH: f32 = 1.;
const STEP_SIZE: f32 = 1.;
const CHEMO_DEPOSIT_AMOUNT: f32 = 5.;
const RANDOM_DIR_CHANGE_P: f32 = 0.;
const SENSITIVITY_MIN_THRESHOLD: f32 = 0.;

pub trait Agent {
    // Default Agent parameters
    fn sensor_angle(&self)              -> f32 { SENSOR_ANGLE.to_radians() }
    fn rotation_angle(&self)            -> f32 { ROTATION_ANGLE.to_radians() }
    fn sensor_offset_distance(&self)    -> f32 { SENSOR_OFFSET_DISTANCE }
    fn sensor_width(&self)              -> f32 { SENSOR_WIDTH }
    fn step_size(&self)                 -> f32 { STEP_SIZE }
    fn chemo_deposit_amount(&self)      -> f32 { CHEMO_DEPOSIT_AMOUNT }
    fn random_dir_change_p(&self)       -> f32 { RANDOM_DIR_CHANGE_P }
    fn sensitivity_min_threshold(&self) -> f32 { SENSITIVITY_MIN_THRESHOLD }

    // Minimum Agent functionality
    fn position(&self) -> Position;
    fn orientation(&self) -> Radians;
    fn move_forward(&mut self);
    fn random_orientation(&mut self);
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn sample(&self, trail_map: &DataMap<TrailCell>) -> (f32, f32, f32);

    // Functions
    fn forward_position(&self) -> Position {
        let (x, y) = self.position().coords();
        let angle = self.orientation();
        Position {
            x: x + angle.cos() * self.step_size(),
            y: y + angle.sin() * self.step_size(),
        }
    }
}

pub struct BasicAgent {
    pub position: Position,
    pub orientation: Radians,
}

impl BasicAgent {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            position: Position::with_random_offset(x as f32, y as f32),
            orientation: random_orientation(),
        }
    }
    pub fn motor_action(&mut self) {}
}

impl Agent for BasicAgent {
    fn position(&self) -> Position {
        self.position.clone()
    }

    fn orientation(&self) -> Radians {
        self.orientation.clone()
    }
    
    fn move_forward(&mut self) {
        self.position = self.forward_position();
    }
    
    fn random_orientation(&mut self) {
        self.orientation = random_orientation();
    }
    
    fn rotate_left(&mut self) {
        self.orientation -= self.rotation_angle();
    }
    
    fn rotate_right(&mut self) {
        self.orientation += self.rotation_angle();
    }
    
    fn sample(&self, trail_map: &DataMap<TrailCell>) -> (f32, f32, f32) {
        // Get the three locations of the sensors
        let position = self.position;
        let front = position.shift(&self.orientation, &self.step_size()).rounded_coords();
        let left  = position.shift(&(&self.orientation - self.sensor_angle()), &self.step_size()).rounded_coords();
        let right = position.shift(&(&self.orientation + self.sensor_angle()), &self.step_size()).rounded_coords();
        let front_sample = trail_map.get(front.0, front.1).map_or(0., |trail_cell| trail_cell.residue);
        let left_sample = trail_map.get(front.0, front.1).map_or(0., |trail_cell| trail_cell.residue);
        let right_sample = trail_map.get(front.0, front.1).map_or(0., |trail_cell| trail_cell.residue);
        (left_sample, front_sample, right_sample)
    }
}