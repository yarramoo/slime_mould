use image; 
use image::GenericImageView;
use rand::Rng;
use std::collections::btree_map::OccupiedEntry;
use std::env;
use std::path::Path;
use slime_mould::DataMap;

const POPULATION: f32 = 0.03;
const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn main() {
    println!("Hello, world!");
    let path = &Path::new("./data");
    let world = World::new(path, 50, 50);
}

mod agent;
use agent::Agent;

#[derive(Clone, Copy, Debug)]
struct EnvironmentCell {
    habitable: bool,
    occupied: bool,
}

const ENV_CELL_MASK_HABITABLE: u8 = 0b1000000;
const ENV_CELL_MASK_OCCUPIED:  u8 = 0b10000000;

impl EnvironmentCell {
    fn new(habitable: bool, occupied: bool) -> Self {
        EnvironmentCell {
            habitable,
            occupied
        }
    }
}

impl From<u8> for EnvironmentCell {
    fn from(value: u8) -> Self {
        let habitable: bool = (value & ENV_CELL_MASK_HABITABLE) != 0b0;
        let occupied:  bool = (value & ENV_CELL_MASK_OCCUPIED) != 0b0;
        Self {
            habitable,
            occupied
        }
    }
}

impl From<EnvironmentCell> for u8 {
    fn from(value: EnvironmentCell) -> Self {
        let mut result = 0;
        if value.habitable {
            result = result | ENV_CELL_MASK_HABITABLE;
        }
        if value.occupied {
            result = result | ENV_CELL_MASK_OCCUPIED;
        }
        result
    }
}

#[derive(Default, Clone, Copy)]
struct TrailCell {
    residue: f32,
}

#[derive(Default)]
struct World {
    agents: Vec<Agent>,
    env_map: DataMap<EnvironmentCell>,
    trail_map: DataMap<TrailCell>,
}

impl World {
    // Create new world. Create new maps
    // TODO?
    //      Allow a way of defining the habitable space
    //      Allow a way of creating different agents at random or according to function/distribution. Currently we can only create one Agent type
    pub fn new(path: &Path, width: usize, height: usize) -> Self {
        // Allocate maps
        let mut env_map = DataMap::new(
            width,
            height,
            vec![EnvironmentCell::new(true, false); width * height]
        );
        let mut trail_map = DataMap::new(
            width,
            height,
            vec![TrailCell::default(); width * height]
        );
        let mut agents = vec![];
        // Initialise maps
        // Populate the env map with agents
        let mut rng = rand::thread_rng();
        for y in 0..height {
            for x in 0..width {
                // Check that the cell is habitable
                let env_cell = env_map
                    .get_mut(x, y)
                    .expect("Should not be out of bounds here");
                if !env_cell.habitable { continue; }
                // Generate a new agent and mark cell occupied
                let r: f32 = rng.gen();
                if r < POPULATION {
                    let agent = Agent::new(x, y);
                    agents.push(agent);
                    env_cell.occupied = true;
                }
            }
        }
        // Create new data map images
        let mut env_img = image::GrayImage::new(width as u32, height as u32);
        let mut trail_img = image::RgbImage::new(width as u32, height as u32);
        // Initialise image data
        // Env image updated with occupied data
        // Trail image stays empty for now
        for (x, y, p) in env_img.enumerate_pixels_mut() {
            let env_cell = env_map
                .get(x as usize, y as usize)
                .expect("Should not be out of bounds here");
            p.0 = [(*env_cell).into()];
        }
        // Save images
        env_img
            .save(path.join("env.png"))
            .expect(format!("Could not save environment image at path {}", path.display()).as_str());
        trail_img
            .save(path.join("trail.png"))
            .expect(format!("Could not save trail image at path {}", path.display()).as_str());
        // return new World
        Self {
            agents, env_map, trail_map
        }
    }
    // Create new world from existing maps
    pub fn from_env_map(path: &Path) -> Self {
        // TODO
        Self::default()
    }
    // Execute a single step of the simulation
    // This will involve a motor stage and sensory stage
    pub fn step(&mut self) {
        self.motor_step();
        self.sensor_step();
    }
    fn motor_step(&mut self) {
        // Each agent tries to move forward
        // If they move, deposit trail
        // If not, choose random new orientation

        // Where can this behaviour be stored? On the agent perhaps? That way each agent can individually determine behaviour
        // What does the world need to know after the motor step?
        //  - It needs the new agent location, and the updated deposit state
        for agent in self.agents.iter_mut() {
            let (x, y) = agent.forward_position().rounded_coords();
            if let Some(envCell) = self.env_map.get(x, y) {
                if envCell.habitable && !envCell.occupied {
                    // Do the motor action
                    continue;
                }
            }
            // Do the non-motor action
        }
    }
    fn sensor_step(&mut self) {
        // Each agent once again

        // Sample trail map
        // Orient towards the strongest trail deposit
    }
}