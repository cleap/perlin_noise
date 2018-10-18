/*
    Christopher Leap
    Perlin Noise
    Algorithm Detail from https://en.wikipedia.org/wiki/Perlin_noise
        Accessed 2018-10-18
*/

extern crate rand;
use rand::prelude::*;
use rand::distributions::Standard;
use std::ops::*;

const GRID_SIZE : usize = 100;

pub struct Perlin2D<T> {
    gradient: [[T; GRID_SIZE]; GRID_SIZE]
}

impl<T> Perlin2D<T> where T: Copy + From<f64>, Standard: Distribution<T>, f64: From<T> {

    fn generate_gradient() -> [[T; GRID_SIZE]; GRID_SIZE] {
        let mut rng = rand::thread_rng();
        let mut gradient = [[rng.gen::<T>(); GRID_SIZE]; GRID_SIZE];
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                grid[x][y] = rng.gen::<T>();
            }
        }
        grid
    }

    fn lerp(a0: T, a1: T, w: f64) -> f64 {
        f64::from(a0) + w * (f64::from(a1) - f64::from(a0))
    }

    fn dotGridGradient(&mut self, ix: usize, iy: usize, x: f64, y: f64) {
        let val: f64 = f64.from(self.grid[ix][iy]);
        // TODO
    }

    pub fn new() -> Perlin2D<T> {
        Perlin2D::<T> {
            gradient: Perlin2D::<T>::generate_gradient()
        }
    }

    pub fn get(&mut self, x: f64, y: f64) -> T {
        let x0: usize = x.floor() as usize;
        let y0: usize = y.floor() as usize;
        self.grid[x0][y0]
    }
}
