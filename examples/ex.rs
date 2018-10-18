extern crate perlin_noise;
use perlin_noise::Perlin2D as Perlin;

extern crate piston_window;
use piston_window::*;

const   STEP : u32 = 10;
const  WIDTH : u32 = 64;
const HEIGHT : u32 = 48;

fn main() {

    let mut perlin: Perlin<f64> = Perlin::new();

    let mut window: PistonWindow =
        WindowSettings::new("Perlin Noise Demo", [WIDTH*STEP, HEIGHT*STEP])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let gray = perlin.get(x as f64, y as f64) as f32;
                    rectangle(
                        [gray, gray, gray, 1.0],
                        [(x*STEP) as f64, (y*STEP) as f64, STEP as f64, STEP as f64],
                        context.transform,
                        graphics);
                }
            }
        });
    }
}
