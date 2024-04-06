use image; 
use image::GenericImageView;
use rand::Rng;
use std::collections::btree_map::OccupiedEntry;
use std::env;
use std::path::{Path, PathBuf};
use slime_mould::DataMap;
use std::thread::sleep;
use std::time::Duration;

const POPULATION: f32 = 0.03;
const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const DIFF_K_SIZE: usize = 3;
const TRAIL_DIFFUSION_DECAY: f32 = 0.1;

fn main() {
    println!("Hello, world!");
    let path = &Path::new("./data");
    let mut world = World::new(path, WIDTH, HEIGHT);
    let mut counter = 0;
    loop {
        world.step();
        counter += 1;
        if counter % 50 == 0 {
            world.save();
        }
    }
}

mod agent;
use agent::{Agent, BasicAgent};

mod cells;
use cells::env_cell::{self, EnvironmentCell};
use cells::trail_cell::TrailCell;

#[derive(Default)]
struct World {
    agents: Vec<Box<dyn Agent>>,
    env_map: DataMap<EnvironmentCell>,
    env_map_swap: DataMap<EnvironmentCell>,
    trail_map: DataMap<TrailCell>,
    trail_map_swap: DataMap<TrailCell>,
    // TODO make these image types dependencies. No need to bake their specific types into the struct
    env_img: image::GrayImage,
    trail_img: image::RgbImage,
    path: PathBuf,
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
        let env_map_swap = DataMap::new(
            width, 
            height,
            vec![EnvironmentCell::default(); width * height]
        );
        let mut trail_map = DataMap::new(
            width,
            height,
            vec![TrailCell::default(); width * height]
        );
        let trail_map_swap = DataMap::new(
            width,
            height,
            vec![TrailCell::default(); width * height]
        );
        let mut agents = vec![];
        // Initialise maps
        // Populate the env map with agents
        // TODO Check if the trail map needs to be initialised with some trial values from placing down the agents
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
                    let agent: Box<dyn Agent> = Box::new(BasicAgent::new(x, y));
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
            agents, 
            env_map, 
            env_map_swap, 
            trail_map, 
            trail_map_swap,
            env_img, 
            trail_img, 
            path: path.into()
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
        self.trail_diffusion();
    }
    fn save(&mut self) {
        self.update_env_map();
        self.update_trail_map();
    }
    fn trail_diffusion(&mut self) {
        // Reduce by scaling factor
        for trail_cell in self.trail_map.data.iter_mut() {
            trail_cell.residue *= (1.0 - TRAIL_DIFFUSION_DECAY);
        }

        // Diffuse along 3x3 kernel
        for x in 0..WIDTH as i32 {
            for y in 0..HEIGHT as i32 {
                let mut neighbour_cells = 0;
                let mut neighbour_residue_total = 0.;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let x = x + dx;
                        let y = y + dy;
                        if x >= 0 && y >= 0 {
                            if let Some(trail_cell) = self.trail_map.get(x as usize, y as usize) {
                                neighbour_cells += 1;
                                neighbour_residue_total += trail_cell.residue;
                            }
                        }
                    }
                }
                let diffuse_residue = neighbour_residue_total / neighbour_cells as f32;
                self.trail_map_swap.get_mut(x as usize, y as usize).unwrap().residue = diffuse_residue;
            }
        }

        std::mem::swap(&mut self.trail_map, &mut self.trail_map_swap);
    }
    fn update_env_map(&mut self) {
        for (x, y, p) in self.env_img.enumerate_pixels_mut() {
            let env_cell = self.env_map
                .get(x as usize, y as usize)
                .expect("Should not be out of bounds here");
            p.0 = [(*env_cell).into()];
        } 
        self.env_img
            .save(self.path.join("env.png"))
            .expect(format!("Could not save environment image at path {}", self.path.display()).as_str());
    }
    fn update_trail_map(&mut self) {
        for (x, y, p) in self.trail_img.enumerate_pixels_mut() {
            let trail_cell = self.trail_map
                .get(x as usize, y as usize)
                .expect("Shouldnot be out of bounds here");
            p.0 = trail_cell.into()
        }
        self.trail_img
            .save(self.path.join("trail.png"))
            .expect(format!("Could not save environment image at path {}", self.path.display()).as_str());
    }
    fn motor_step(&mut self) {
        // Each agent tries to move forward
        // If they move, deposit trail
        // If not, choose random new orientation

        // Reset the swap map
        // TODO look into a nicer way of calling iter() on the struct instead of the data field...
        for swap_env_cell in self.env_map_swap.data.iter_mut() {
            swap_env_cell.occupied = false;
        }

        // Where can this behaviour be stored? On the agent perhaps? That way each agent can individually determine behaviour
        // What does the world need to know after the motor step?
        //  - It needs the new agent location, and the updated deposit state
        // TODO make this loop random in order
        for agent in self.agents.iter_mut() {
            let (x, y) = agent.forward_position().rounded_coords();
            // If the next cell is valid (exists + habitable + unoccupied both in current and next step) then go to it
            // Else stay and mark the next step in the same position as occupied
            if let Some(env_cell) = self.env_map.get_mut(x, y) {
                let next_env_cell = self.env_map_swap.get_mut(x, y).unwrap();
                if next_env_cell.habitable && !next_env_cell.occupied && !env_cell.occupied {
                    agent.move_forward();
                    next_env_cell.occupied = true;
                    env_cell.occupied = false;
                    self.trail_map.get_mut(x, y).unwrap().residue += agent.chemo_deposit_amount();
                    continue;
                }
            }
            let (old_x, old_y) = agent.position().rounded_coords();
            let env_cell = self.env_map_swap.get_mut(old_x, old_y).unwrap();
            env_cell.occupied = true;
            agent.random_orientation();
        }

        std::mem::swap(&mut self.env_map, &mut self.env_map_swap)
    }
    fn sensor_step(&mut self) {
        // Each agent once again
        for agent in self.agents.iter_mut() {
            // Sample trail map
            let (left_s, middle_s, right_s) = agent.sample(&self.trail_map);
            // Orient towards the strongest trail deposit
            if middle_s < left_s && middle_s < right_s {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    agent.rotate_right();
                } else {
                    agent.rotate_left();
                }
            }
            else if left_s < right_s {
                agent.rotate_right();
            }
            else if right_s < left_s {
                agent.rotate_left();
            }
        }
    }
}