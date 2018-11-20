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

    fn lerp(a0: f64, a1: f64, w: f64) -> f64 {
        a0 + w * (a1 - a0)
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

        // Determine cell coordinates
        let x0: usize = x.floor() as usize % GRID_SIZE;
        let x1: usize = x0 + 1;
        let y0: usize = y.floor() as usize % GRID_SIZE;
        let y1: usize = y0 + 1;

        // Determine interpolation weights
        let sx: f64 = x - x0 as f64;
        let sy: f64 = y - y0 as f64;

        // Interpolate between grid point gradients
        let n0 = self.dot_grid_gradient(x0, y0, x, y);
        let n1 = self.dot_grid_gradient(x1, y0, x, y);
        let ix0 = Perlin2D::lerp(n0, n1, sx);
        let n0 = self.dot_grid_gradient(x0, y1, x, y);
        let n1 = self.dot_grid_gradient(x1, y1, x, y);
        let ix1 = Perlin2D::lerp(n0, n1, sx);
        T::from(Perlin2D::lerp(ix0, ix1, sy))
    }
}
