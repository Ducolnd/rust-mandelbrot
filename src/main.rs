use gif::Encoder;
use std::fs;
use std::fs::File;
use Vec;
use std::{thread};

mod helper;
mod constants;
mod brot;

use constants::*;

fn main() -> std::io::Result<()>{

    let colors = helper::init_colors("colors.txt");

    let mut threads: Vec<_> = Vec::new();
    let range: Vec<i32> = (START_ZOOM..END_ZOOM).collect();

    let output = File::create(format!("render/gif/{}.gif", SET_NAME)).unwrap();
    let mut gif = Encoder::new(output, WIDTH as u16, HEIGHT as u16, &[]).unwrap();
    gif.set_repeat(gif::Repeat::Infinite).unwrap();
    
    fs::create_dir_all(format!("render/{}", SET_NAME))?;

    for x in range {
        let c = colors.clone();

        // Create a thread for every frame
        threads.push(thread::spawn(move || {
            println!("Started with {}", x);
            let imgbuf = brot::brot(x, 0.3602404434376143632361252444495453084826078079585857504883758147401953460592181003117529367227734263962337317297249877373200353726832853176645324012185215795, -0.6413130610648031748603750151793020665794949522823052595561775430644485741727536902556370230689681162370740565537072149790106973211105273740851993394803287437606238596262, &c);
            println!("Rendered {}", x);

            imgbuf.save(format!("render/{}/frame-{:04}.png", SET_NAME, x)).unwrap();

            // let mut gifframe = gif::Frame::from_rgb_speed(WIDTH as u16, HEIGHT as u16, &imgbuf.into_vec(), 28);
            // gifframe.delay = FRAME_TIME;
            // println!("Loaded frame {}", x);
            
            // return gifframe
        }));
    }

    // Wait for the threads to finish
    for handle in threads {
        let frame = handle.join().unwrap();

        // gif.write_frame(&frame);
    } 
    
    Ok(())
}