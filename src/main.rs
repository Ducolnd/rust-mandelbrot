use image;
use gif::Encoder;
use std::fs::File;
use Vec;
use std::{thread};

mod helper;
mod constants;
mod brot;

use constants::*;

fn main() {
    let colors = helper::init_colors("colors.txt");

    let mut threads: Vec<_> = Vec::new();
    let range: Vec<u32> = (0..MAX_ZOOM as u32).collect();

    let output = File::create(format!("render/gif/{}.gif", SET_NAME)).unwrap();
    let mut gif = Encoder::new(output, WIDTH as u16, HEIGHT as u16, &[]).unwrap();
    gif.set_repeat(gif::Repeat::Infinite).unwrap();
    
    for x in range {
        let c = colors.clone();

        threads.push(thread::spawn(move || {
            println!("Started with {}", x);
            let imgbuf = brot::brot(x, 0.3602404434376143632361252444495453084826078079585857504883758147401953460592181003117529367227734263962337317297249877373200353726832853176645324012185215795, -0.6413130610648031748603750151793020665794949522823052595561775430644485741727536902556370230689681162370740565537072149790106973211105273740851993394803287437606238596262, &c);
            println!("Ended with {}", x);

            return imgbuf
        }));
    }

    // Wait for the threads to finish and order them
    for handle in threads {
        let buf = handle.join().unwrap();

        println!("Load a frame");
        let mut gifframe = gif::Frame::from_rgb_speed(WIDTH as u16, HEIGHT as u16, &buf.into_vec(), 30);
        gifframe.delay = FRAME_TIME;
    
        println!("Wrote a frame");
        gif.write_frame(&gifframe);
    }    
}