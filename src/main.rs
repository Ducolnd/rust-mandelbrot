use num::Complex;
use image;
use Vec;

mod helper;

// Setup constants
const MAX_ITERATIONS: u32 = 100;
const SIZE: usize = 800;
const VIEWMINX: f64 = -2.25;    // X
const VIEWMAXX: f64 = 0.75;
const VIEWMINY: f64 = -1.25;     // Y
const VIEWMAXY: f64 = 1.25;


// Mandelbrot z' = z^2 + c
fn f(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z.powu(2) + c
}

// Get numer of iterations required
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
fn to_rgb(it: u32) -> u8 {
    if it == MAX_ITERATIONS { 255 } 
    else { 0 }
}

fn main() {
    let re = helper::linspace(VIEWMINX, VIEWMAXX, SIZE);
    let im = helper::linspace(VIEWMINY, VIEWMAXY, SIZE);

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
    let colors: Vec<u8> = itered.iter().map(|x| to_rgb(*x)).collect();

    let mut imgbuf = image::GrayImage::new(SIZE as u32, SIZE as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([colors[(x * SIZE as u32 + y) as usize]]);
    }

    imgbuf.save("mandelbrot.png").unwrap();
}