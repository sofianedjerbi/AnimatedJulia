mod algo;
mod graphics;

use image::{RgbImage};

use crate::graphics::{WIDTH, HEIGHT,
                      print_julia,
                      print_mandelbrot};

use std::time::Instant;
use std::{env, fs};


fn print_help() {
    println!("Usage: julia [MODE]");
    println!("Modes:");
    println!("    \"-jd\": Display a simple Julia fractal");
    println!("    \"-jr\": Render a Julia rotation GIF //TODO");
    println!("    \"-md\": Display the Mandelbrot fractal //TODO");
    println!("    \"-mz\": Zoom on the Mandelbrot fractal //TODO");
}

fn mandelbrot_display() {
    // New image
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    print_mandelbrot(&mut img, -1.0, 0.0); // Print julia fractal on img

    println!("Saving Mandelbrot."); // Save
    match img.save("mandelbrot.png") {
        Ok(s) => s,
        Err(e) => panic!("Cannot save fractal.\n{:?}", e)
    };
}

fn julia_display(a: f32, b: f32, name: &str) {
    // New image
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    print_julia(&mut img, a, b); // Print julia fractal on img

    println!("Saving Julia for c = {} + {}i", a, b); // Save
    match img.save(name) {
        Ok(s) => s,
        Err(e) => panic!("Cannot save fractal.\n{:?}", e)
    };


}

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 && (args.len() != 2 || args[1] != "-md") {
        print_help();
        return
    }
    
    // Create directory
    match fs::create_dir_all("./render.tmp/") {
        Ok(s) => s,
        Err(e) => panic!("Cannot create render directory here.\n{:?}", e)
    };

    // Get mode
    let mode = &args[1];

    // Benchmark 
    let before = Instant::now();
    println!("Computing...");
    
    if mode == "-md" {              // Mandelbrot
        mandelbrot_display();
    }
    else if mode == "-jd" {         // Julia
        // Grabbing real and imaginary part
        let a: f32 = match args[2].parse::<f32>() {
            Ok(s) => s,
            Err(e) => panic!("Invalid argument [REAL PART].\n{:?}", e)
        };
        let b: f32 = match args[3].parse::<f32>() {
            Ok(s) => s,
            Err(e) => panic!("Invalid argument [IMAGINARY PART].\n{:?}", e)
        };
        julia_display(a, b, "julia.png");
    } else {
        print_help();
        return
    }

    let now = Instant::now(); // Finish benchmark and print
    println!("Done ({:?})", now.duration_since(before));
}


