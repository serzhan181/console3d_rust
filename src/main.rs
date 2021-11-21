pub mod helpers;
use helpers::VecUv;
use helpers::{sum_x, sum_y};

const WIDTH: usize = 120;
const HEIGHT: usize = 30;
const ASPECT: f64 = WIDTH as f64 / HEIGHT as f64;
const PIXEL_ASPECT: f64 = 11.0 / 24.0;

fn main() {
    let gradient = vec![
        ' ', '.', ':', '!', '/', 'r', '(', 'l', '1', 'Z', '4', 'H', '9', 'W', '8', '$', '@',
    ];
    let gradient_size = gradient.len() - 2;

    let mut screen = vec![' '; WIDTH * HEIGHT];

    for t in 0..10000 {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let mut uv = VecUv {
                    x: sum_x(i, WIDTH),
                    y: sum_y(j, HEIGHT),
                };

                uv.x *= ASPECT * PIXEL_ASPECT;
                uv.x += f64::sin(t as f64 * 0.001);

                let dist = (uv.x * uv.x + uv.y * uv.y).sqrt();
                let mut color = (1.0f64 / dist) as i32;

                color = color.clamp(0, gradient_size as i32);

                let pixel = gradient[color as usize];

                screen[i + j * WIDTH] = pixel;
            }
        }
        println!("{}", screen.clone().into_iter().collect::<String>());
    }
}
