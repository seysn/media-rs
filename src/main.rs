use media_rs::{bmp::BMP, image::ImageDecoder};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bmp = BMP::read_raw(&args[1]);
    println!("{:?}", bmp);
}
