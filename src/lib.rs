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

// #[derive(Clone, Copy)]
// pub struct Orientation {
//     pub radians: f32,
// }

// impl Orientation {
//     pub fn new(radians: f32) -> Self {
//         Self { radians }
//     }
//     pub fn new_random() -> Self {
//         let mut rng = rand::thread_rng();
//         let angle: f32 = rng.gen_range(0.0..2.);
//         Self::new(angle)
//     }
// }

// impl Add for Orientation {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             radians: self.radians + rhs.radians
//         }
//     }
// }

// impl Sub for Orientation {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             radians: self.radians - rhs.radians
//         }
//     }
// }

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
        self.data.get(self.width * y + x)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(self.width * y + x)
    }
}

// impl<T: Into<u8>>

impl<T: Debug> Default for DataMap<T> {
    fn default() -> Self {
        Self { width: 0, height: 0, data: Default::default() }
    }
}

// struct DataMapIterator<'a, T> {
//     data: &'a Vec<T>,
//     index: usize,
// }

// impl <'a, T> Iterator for DataMapIterator<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
        
//     }
// }



// impl<T> IntoIterator for DataMap<T> {
//     type Item = T;

//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.data.into_iter()
//     }
// }