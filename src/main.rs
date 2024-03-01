use opencv::core::Mat;
use opencv::videoio::CAP_ANY;
use opencv::videoio::VideoCapture;

use opencv::prelude::VideoCaptureTrait;

mod lib;
use lib::*;

fn main() {
    println!("Hello, world!");
    for index in 0..10 {
        let mut cam = VideoCapture::new(index, CAP_ANY);
        if let Ok(mut cam) = cam {
            let mut frame = Mat::default();
            cam.read(&mut frame);
            println!("{:?}", frame);
            println!("index {index}");
        }
    }
    let _ = detect_loop_hybrid(0, true, 30.0);
    loop {}
}
