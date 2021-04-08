use num::Complex;
use image;
use Vec;
use std::{thread};
use std::fs;
use image::imageops;

mod helper;
mod constants;

use constants::*;

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

fn brot(zoom: f64, zoom_point_x: f64, zoom_point_y: f64, with_colors: &Vec<[u8; 3]>) {
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
    let colors: Vec<[u8; 3]> = itered.iter().map(|x| helper::to_rgb(*x, with_colors)).collect();

    let mut imgbuf = image::ImageBuffer::new(SIZE as u32, SIZE as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = &colors[(x * SIZE as u32 + y) as usize];
        *pixel = image::Rgb(*color);
    }


    imgbuf = imageops::unsharpen(&imgbuf, 0.8, 2);
    
    fs::create_dir_all(format!("render/{}/", SET_NAME));
    imgbuf.save(format!("render/{}/mandelbrot-{}.png", SET_NAME, zoom)).unwrap();

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
    let colors = helper::init_colors("colors.txt");


    let mut threads: Vec<_> = Vec::new();
    let range = helper::linspace(1.0, MAX_ZOOM, IMAGES);
    
    for x in range {
        let c = colors.clone();

        threads.push(thread::spawn(move || {
            println!("Started with {}", x);
            brot(0.8 + (f64::powf(x as f64, 6.0) / 10000.0), 0.3602404434376143632361252444495453084826078079585857504883758147401953460592181003117529367227734263962337317297249877373200353726832853176645324012185215795, -0.6413130610648031748603750151793020665794949522823052595561775430644485741727536902556370230689681162370740565537072149790106973211105273740851993394803287437606238596262, &c);
            println!("Ended with {}", x);
        }));
    }

    for handle in threads {
        handle.join().unwrap()
    }
}