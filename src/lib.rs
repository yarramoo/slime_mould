use std::{fmt::Debug, ops::{Add, Sub}, path::Iter};
use rand::Rng;

pub type Radians = f32;

pub fn random_orientation() -> Radians {
    let upper = 2. * std::f32::consts::PI;
    rand::thread_rng().gen_range(0.0..upper)
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn with_random_offset(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let dx: f32 = rng.gen();
        let dy: f32 = rng.gen();
        Position::new(x + dx, y + dy)
    }
    pub fn rounded_coords(&self) -> (usize, usize) {
        (self.x.floor() as usize, self.y.floor() as usize)
    }
    pub fn coords(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    pub fn shift(&self, orientation: &Radians, distance: &f32) -> Position {
        Position::new(
            self.x + distance * orientation.cos(),
            self.y + distance * orientation.sin(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct DataMap<T: Debug> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Debug> DataMap<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self { width, height, data }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height { return None; }
        return Some(unsafe { self.data.get_unchecked(self.width * y + x)} );
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height { return None; }
        return Some(unsafe { self.data.get_unchecked_mut(self.width * y + x)} );
    }
}

impl<T: Debug> Default for DataMap<T> {
    fn default() -> Self {
        Self { width: 0, height: 0, data: Default::default() }
    }
}
