use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn rounded_coords(&self) -> (usize, usize) {
        (self.x.floor() as usize, self.y.floor() as usize)
    }
    pub fn coords(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Clone, Copy)]
pub struct Orientation {
    pub angle: f32,
}

impl Orientation {
    pub fn new(angle: f32) -> Self {
        assert!(angle >= 0. && angle < 360.);
        Self { angle }
    }
}

impl Add for Orientation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            angle: self.angle.rem_euclid(rhs.angle)
        }
    }
}

pub struct DataMap<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> DataMap<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self { width, height, data }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(self.width * y + x)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(self.width * y + x)
    }
}

impl<T> Default for DataMap<T> {
    fn default() -> Self {
        Self { width: 0, height: 0, data: Default::default() }
    }
}