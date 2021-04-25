use num::Complex;
use Vec;

use crate::helper;
use crate::constants::*;

// Mandelbrot z' = z^2 + C
fn f(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z.powu(2) + c
}

#[cfg(feature = "linear")]
fn iter(c: Complex<f64>) -> f64 {
    let mut result = Complex::new(0.0, 0.0);
    let mut n = 0;
    let mut pow = 1.0;
    let mut norm = 0.0;

    loop {
        result = f(result, c);
        n += 1;
        norm = result.norm();

        if (n >= MAX_ITERATIONS) || (result.norm() > 1000.0) {
            break
        }
    }

    (norm * norm).log(10.0) / 2.0
}

#[cfg(feature = "non-linear")]
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

pub fn brot(zoom: i32, zoom_point_x: f64, zoom_point_y: f64, with_colors: &Vec<[u8; 3]>) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let factor = WIDTH as f64 / HEIGHT as f64;
    let half_width = WIDTH as f64 / 2.0;
    let half_height = HEIGHT as f64 / 2.0;
    let deletion = f64::powf(DELETION_FACTOR, zoom as f64);

    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Determine coordinates for z = a + bi
        let a = zoom_point_x + ((x as f64 - WIDTH as f64 / 2.0) / half_width) * 2.0 * factor * deletion;
        let b = zoom_point_y + ((y as f64 - HEIGHT as f64 / 2.0) / half_height) * 2.0 * deletion;

        // Determine color of coordinate from the number of iterations
        #[cfg(feature = "linear")]
        let color = helper::to_rgb(iter(Complex::new(a, b)));
        #[cfg(feature = "non-linear")]
        let color = helper::to_rgb(iter(Complex::new(a, b)), with_colors);

        *pixel = image::Rgb(color);
    }
    
    imgbuf    
}