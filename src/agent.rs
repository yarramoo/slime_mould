use rand::Rng;
use slime_mould::{Position, Orientation};

const SENSOR_ANGLE: f32 = 22.5;
const ROTATION_ANGLE: f32 = 45.;
const SENSOR_OFFSET_DISTANCE: usize = 9;
const SENSOR_WIDTH: usize = 1;
const STEP_SIZE: usize = 1;
const CHEMO_DEPOSIT_AMOUNT: usize = 5;
const RANDOM_DIR_CHANGE_P: f32 = 0.;
const SENSITIVITY_MIN_THRESHOLD: f32 = 0.;

pub struct Agent {
    pub position: Position,
    pub orientation: Orientation,
}

impl Agent {
    pub fn motor_action(&mut self) {}
    pub fn forward_position(&self) -> Position {
        let (px, py) = self.position.coords();
        let angle = self.orientation.angle;
        Position {
            x: px + angle.cos() * px,
            y: py + angle.cos() * py,
        }
    }
}