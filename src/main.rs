use image;
use Vec;
use std::{thread};
use std::fs;
use image::imageops;

mod helper;
mod constants;
mod brot;

use constants::*;

fn main() {
    let colors = helper::init_colors("colors.txt");

    let mut threads: Vec<_> = Vec::new();
    let range = helper::linspace(1.0, MAX_ZOOM, IMAGES);
    
    for x in range {
        let c = colors.clone();

        threads.push(thread::spawn(move || {
            println!("Started with {}", x);
            let imgbuf = brot::brot(0.8 + (f64::powf(x as f64, 6.0) / 10000.0), 0.3602404434376143632361252444495453084826078079585857504883758147401953460592181003117529367227734263962337317297249877373200353726832853176645324012185215795, -0.6413130610648031748603750151793020665794949522823052595561775430644485741727536902556370230689681162370740565537072149790106973211105273740851993394803287437606238596262, &c);

            fs::create_dir_all(format!("render/{}/", SET_NAME));
            imgbuf.save(format!("render/{}/mandelbrot-{}.png", SET_NAME, x)).unwrap();

            println!("Ended with {}", x);
        }));
    }

    for handle in threads {
        handle.join().unwrap()
    }
}