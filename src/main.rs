use image;
use gif::Encoder;
use std::fs::File;
use Vec;
use std::{thread};
use std::fs;

mod helper;
mod constants;
mod brot;

use constants::*;

fn main() {
    let colors = helper::init_colors("colors.txt");

    let mut threads: Vec<_> = Vec::new();
    let range = helper::linspace(1.0, MAX_ZOOM, IMAGES);

    let mut output = File::create(format!("render/gif/{}.gif", SET_NAME)).unwrap();
    let mut gif = Encoder::new(output, SIZE as u16, SIZE as u16, &[]).unwrap();
    gif.set_repeat(gif::Repeat::Infinite).unwrap();

    
    for x in range {
        let c = colors.clone();

        threads.push(thread::spawn(move || {
            println!("Started with {}", x);
            let imgbuf = brot::brot(0.8 + (f64::powf(x as f64, 6.0) / 10000.0), 0.3602404434376143632361252444495453084826078079585857504883758147401953460592181003117529367227734263962337317297249877373200353726832853176645324012185215795, -0.6413130610648031748603750151793020665794949522823052595561775430644485741727536902556370230689681162370740565537072149790106973211105273740851993394803287437606238596262, &c);
            println!("Ended with {}", x);

            return imgbuf
        }));
    }

    let mut frames = Vec::<image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>>::new();

    // Wait for the threads to finish
    for handle in threads {
        frames.push(handle.join().unwrap());
    }    
    
    for frame in frames {
        println!("Load a frame");
        let mut frame = gif::Frame::from_rgb_speed(SIZE as u16, SIZE as u16, &frame.into_vec(), 10);
        frame.delay = 10;
    
        println!("Wrote a frame");
        gif.write_frame(&frame);
    }
}