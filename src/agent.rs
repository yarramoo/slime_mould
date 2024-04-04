use rand::Rng;
use slime_mould::{Position, Orientation};

const SENSOR_ANGLE: f32 = 22.5;
const ROTATION_ANGLE: f32 = 45.;
const SENSOR_OFFSET_DISTANCE: f32 = 9.;
const SENSOR_WIDTH: f32 = 1.;
const STEP_SIZE: f32 = 1.;
const CHEMO_DEPOSIT_AMOUNT: usize = 5;
const RANDOM_DIR_CHANGE_P: f32 = 0.;
const SENSITIVITY_MIN_THRESHOLD: f32 = 0.;

pub trait Agent {
    // Default Agent parameters
    fn sensor_angle(&self)              -> f32 { SENSOR_ANGLE }
    fn rotation_angle(&self)            -> f32 { ROTATION_ANGLE }
    fn sensor_offset_distance(&self)    -> f32 { SENSOR_OFFSET_DISTANCE }
    fn sensor_width(&self)              -> f32 { SENSOR_WIDTH }
    fn step_size(&self)                 -> f32 { STEP_SIZE }
    fn chemo_deposit_amount(&self)      -> usize { CHEMO_DEPOSIT_AMOUNT }
    fn random_dir_change_p(&self)       -> f32 { RANDOM_DIR_CHANGE_P }
    fn sensitivity_min_threshold(&self) -> f32 { SENSITIVITY_MIN_THRESHOLD }

    // Minimum Agent functionality
    fn position(&self) -> Position;
    fn orientation(&self) -> Orientation;
    fn forward_position(&self) -> Position {
        let (x, y) = self.position().coords();
        let angle = self.orientation().angle;
        Position {
            x: x + angle.cos() * self.step_size(),
            y: y + angle.cos() * self.step_size(),
        }
    }
}

pub struct BasicAgent {
    pub position: Position,
    pub orientation: Orientation,
}

impl BasicAgent {
    pub fn new(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: Position {
                x: x as f32 + rng.gen::<f32>(),
                y: y as f32 + rng.gen::<f32>(),
            },
            orientation: Orientation {
                angle: rng.gen_range(0.0..360.0),
            }
        }
    }
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

impl Agent for BasicAgent {
    fn position(&self) -> Position {
        self.position.clone()
    }

    fn orientation(&self) -> Orientation {
        self.orientation.clone()
    }
}