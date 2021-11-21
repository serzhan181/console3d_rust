pub struct VecUv {
    pub x: f64,
    pub y: f64,
}

pub fn sum_x(i: usize, width: usize) -> f64 {
    i as f64 / width as f64 * 2.0f64 - 1.0f64
}

pub fn sum_y(j: usize, height: usize) -> f64 {
    j as f64 / height as f64 * 2.0f64 - 1.0f64
}
