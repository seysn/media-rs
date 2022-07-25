use media_rs::bmp::BMP;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bmp = BMP::read(args[1].to_string());
    println!("{:?}", bmp);
}
