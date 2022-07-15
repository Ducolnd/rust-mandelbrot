use std::io::{BufRead, BufReader};
use std::{fs::File, u8};

use raster::Color;

use crate::constants::MAX_ITERATIONS;

#[cfg(feature = "linear")]
// This produces the nice fractal
pub fn to_rgb(it: f64) -> [u8; 3] {
    if !(it < MAX_ITERATIONS as f64) {
        return [0, 0, 0];
    }

    let c: f64 =
        (1.0 as f64 / ((7.0 * 3.0 as f64).powf(1.0 / 8.0))) * (1.0 / std::f64::consts::LOG2_10);

    let r = 255.0 * ((1.0 - (A * it).cos()) / 2.0);
    let g = 255.0 * ((1.0 - (B * it).cos()) / 2.0);
    let b = 255.0 * ((1.0 - (c * it).cos()) / 2.0);

    // print!(" {:?} ", [r, g, b]);

    [r as u8, b as u8, g as u8]
}

#[cfg(feature = "non-linear")]
// This produces the nice fractal
pub fn to_rgb(it: u32, colors: &Vec<[u8; 3]>) -> [u8; 3] {
    if !(it < MAX_ITERATIONS) {
        return [0, 0, 0];
    }

    let i = it as usize % colors.len();

    return colors[i];
}

pub fn init_colors(path: &str) -> Vec<[u8; 3]> {
    let mut v = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let c = Color::hex(&line.unwrap()).unwrap();

        v.push([c.r, c.g, c.b])
    }

    let mut b = v.clone();
    b.reverse();

    v.append(&mut b);

    v
}
