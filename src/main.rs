use image; 
use image::GenericImageView;
use std::path::Path;
use slime_mould::DataMap;

fn main() {
    println!("Hello, world!");
    let path = &Path::new("./data");
    let world = World::new(path, 50, 50);
}

mod agent;
use agent::Agent;

#[derive(Default, Clone, Copy)]
struct EnvironmentCell {
    habitable: bool,
    occupied: bool,
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
    // Allow a way of defining the habitable space
    pub fn new(path: &Path, width: usize, height: usize) -> Self {
        // TODO
        // Initialise maps
        let mut env_map = DataMap::new(
            width,
            height,
            vec![EnvironmentCell::default(); width * height]
        );
        let mut trail_map = DataMap::new(
            width,
            height,
            vec![TrailCell::default(); width * height]
        );
        let mut agents = vec![];

        // Create new data map images
        // TODO actually save map data to images
        let mut env_img = image::GrayImage::new(width as u32, height as u32);
        let mut trail_img = image::RgbImage::new(width as u32, height as u32);
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