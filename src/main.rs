use num::Complex;
use image;
use Vec;
use std::{thread, time};
use std::f64::consts::E;

mod helper;

// Setup constants
const MAX_ITERATIONS: u32 = 400;
const SIZE: usize = 800;

// Mandelbrot z' = z^2 + C
fn f(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z.powu(2) + c
}

// Get numer of iterations required for any complex number C
fn iter(c: Complex<f64>) -> u32 {
    let mut result = Complex::new(0.0, 0.0);
    let mut n = 0;

    loop {
        result = f(result, c);
        n += 1;

        if (n >= MAX_ITERATIONS) || (result.norm() > 2.0) {
            break
        }
    }

    n
}

// This produces the nice fractal
fn to_rgb(it: u32) -> Vec<u8> {
    if !(it < MAX_ITERATIONS) {return vec![0, 0, 0]}

    let i = it % 16;

    match i {
        0 => vec![66, 30, 15],
        1 => vec![25, 7, 26],
        2 => vec![9, 1, 47],
        3 => vec![4, 4, 73],
        4 => vec![0, 7, 100],
        5 => vec![12, 44, 138],
        6 => vec![24, 82, 177],
        7 => vec![57, 125, 209],
        8 => vec![134, 181, 229],
        9 => vec![211, 236, 248],
        10 => vec![241, 233, 191],
        11 => vec![248, 201, 95],
        12 => vec![255, 170, 0],
        13 => vec![204, 128, 0],
        14 => vec![153, 87, 0],
        15 => vec![106, 52, 3],
        _ => vec![0, 0, 0],
    }
}

fn brot(zoom: f64, zoom_point_x: f64, zoom_point_y: f64) {
    let re = helper::linspace(zoom_point_x - (2.0 / zoom), zoom_point_x + (1.0 / zoom), SIZE);
    let im = helper::linspace(zoom_point_y - (1.5 / zoom), zoom_point_y + (1.5 / zoom), SIZE);

    let mut nums: Vec<Complex<f64>> = Vec::new();
    nums.resize(SIZE * SIZE, Complex::new(0.0, 0.0));

    // a + bi
    for (posa, a) in re.iter().enumerate() {
        for (posb, b) in im.iter().enumerate() {
            nums[posa * SIZE + posb] = Complex::new(*a, *b);
        }
    }

    let itered: Vec<_> = nums.iter().map(|x| iter(*x)).collect(); // Iterated complex nums 

    // Create and write image
    let colors: Vec<Vec<u8>> = itered.iter().map(|x| to_rgb(*x)).collect();

    let mut imgbuf = image::ImageBuffer::new(SIZE as u32, SIZE as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = &colors[(x * SIZE as u32 + y) as usize];
        *pixel = image::Rgb([
            color[0],
            color[1],
            color[2],
        ]);
    }

    imgbuf.save(format!("render/mandelbrot-{}.png", zoom)).unwrap();

    println!(
        "Rendered mendelbrot. Re min: {} Re max: {} Im min: {} Im max: {} at {}", 
        zoom_point_x - (2.0 / zoom), 
        zoom_point_x + (1.0 / zoom), 
        zoom_point_y - (1.5 / zoom), 
        zoom_point_y + (1.5 / zoom),
        format!("render/mandelbrot-{}.png", zoom)
    );
}

fn main() {
    for i in (1..20).step_by(1) {
        brot((i * i * i) as f64, -0.77568377, 0.13646737);
    }
}