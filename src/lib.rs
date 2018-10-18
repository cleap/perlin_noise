/*
    Christopher Leap
    Perlin Noise
    Algorithm Detail from https://en.wikipedia.org/wiki/Perlin_noise
        Accessed 2018-10-18
*/

extern crate rand;
use rand::prelude::*;
use rand::distributions::Standard;

const GRID_SIZE : usize = 100;

pub struct Perlin2D<T> {
    gradient: [[[T; 2]; GRID_SIZE]; GRID_SIZE]
}

impl<T> Perlin2D<T> where T: Copy + From<f64>, Standard: Distribution<T>, f64: From<T> {

    fn generate_gradient() -> [[[T; 2]; GRID_SIZE]; GRID_SIZE] {
        let mut rng = rand::thread_rng();
        let mut gradient = [[[rng.gen::<T>(); 2]; GRID_SIZE]; GRID_SIZE];
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                gradient[x][y][0] = rng.gen::<T>();
                gradient[x][y][1] = rng.gen::<T>();
            }
        }
        gradient
    }

    fn lerp(a0: T, a1: T, w: f64) -> f64 {
        f64::from(a0) + w * (f64::from(a1) - f64::from(a0))
    }

    fn dot_grid_gradient(&mut self, ix: usize, iy: usize, x: f64, y: f64) -> f64 {
        let val0: f64 = f64::from(self.gradient[ix][iy][0]);
        let val1: f64 = f64::from(self.gradient[ix][iy][1]);
        let dx: f64 = x - ix as f64;
        let dy: f64 = y - iy as f64;

        dx*val0 + dy*val1
    }

    pub fn new() -> Perlin2D<T> {
        Perlin2D::<T> {
            gradient: Perlin2D::<T>::generate_gradient()
        }
    }

    pub fn get(&mut self, x: f64, y: f64) -> T {
        let x0: usize = x.floor() as usize;
        let y0: usize = y.floor() as usize;
        self.gradient[x0][y0][0]
    }
}
