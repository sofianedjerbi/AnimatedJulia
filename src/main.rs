mod julia;
mod graphics;

use image::{RgbImage};

use crate::graphics::{print_julia, WIDTH, HEIGHT};

use std::time::Instant;
use std::{env, fs};


fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: julia [REAL PART] [IMAGINARY PART]");
        return
    }

    // Creating render directory
    match fs::create_dir_all("./render.tmp/") {
        Ok(s) => s,
        Err(e) => panic!("Cannot create render directory here.\n{:?}", e)
    };

    // Grabbing real and imaginary part
    let a: f32 = match args[1].parse::<f32>() {
        Ok(s) => s,
        Err(e) => panic!("Invalid argument [REAL PART].\n{:?}", e)
    };
    let b: f32 = match args[2].parse::<f32>() {
        Ok(s) => s,
        Err(e) => panic!("Invalid argument [IMAGINARY PART].\n{:?}", e)
    };
    
    // New image
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    
    let before = Instant::now(); // Benchmark
    println!("Computing...");
    
    print_julia(&mut img, a, b); // Print julia fractal on img 

    println!("Saving Julia for c = {} + {}i", a, b); // Save 
    match img.save("./render.tmp/fractal.png") {
        Ok(s) => s,
        Err(e) => panic!("Cannot save fractal.\n{:?}", e)
    };
    
    let now = Instant::now(); // Finish benchmark and print
    println!("Done ({:?})", now.duration_since(before));
}
