use num::Complex;
use Vec;

use crate::helper;
use crate::constants::*;

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

pub fn brot(zoom: u32, zoom_point_x: f64, zoom_point_y: f64, with_colors: &Vec<[u8; 3]>) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let factor = WIDTH as f64 / HEIGHT as f64;

    let re = helper::linspace(
        zoom_point_x - (2.0 * factor * f64::powf(DELETION_FACTOR, zoom as f64)), 
        zoom_point_x + (2.0 * factor * f64::powf(DELETION_FACTOR, zoom as f64)), 
        WIDTH
    );

    let im = helper::linspace(
        zoom_point_y - (2.0 * f64::powf(DELETION_FACTOR, zoom as f64)), 
        zoom_point_y + (2.0 * f64::powf(DELETION_FACTOR, zoom as f64)), 
        HEIGHT
    );

    let mut nums: Vec<Complex<f64>> = Vec::new();
    nums.resize(WIDTH * HEIGHT, Complex::new(0.0, 0.0));

    // a + bi
    for (posb, b) in im.iter().enumerate() {
        for (posa, a) in re.iter().enumerate() {
            nums[posa * HEIGHT + posb] = Complex::new(*a, *b);
        }
    }

    let itered: Vec<_> = nums.iter().map(|x| iter(*x)).collect(); // Iterated complex nums 

    // Create and write image
    let colors: Vec<[u8; 3]> = itered.iter().map(|x| helper::to_rgb(*x, with_colors)).collect();

    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = &colors[(x * HEIGHT as u32 + y) as usize];
        *pixel = image::Rgb(*color);
    }
    
    imgbuf    
}