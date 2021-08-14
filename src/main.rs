mod algo;
mod graphics;

use libm::tanh;
use image::RgbImage;
use rayon::prelude::*;

use crate::graphics::{WIDTH, HEIGHT,
                      print_julia,
                      print_mandelbrot};

use std::f32::consts::PI;
use std::time::Instant;
use std::{env, fs};


const ITER_ROTATION: u16 = 1200; // Frames for the julia animation  
const ITER_ZOOM: u16 = 3600; // Frames for mandelbrot zoom (1 min)

fn print_help() {
    println!("Usage: julia [MODE]");
    println!("Modes:");
    println!("    \"-jd <re(c)> <im(c)>\": Display a simple Julia fractal");
    println!("    \"-jr <norm(c)>\": Render a Julia rotation GIF");
    println!("    \"-md\": Display the Mandelbrot fractal");
    println!("    \"-mz <re(c)> <im(c)>\": Zoom on the Mandelbrot fractal");
}

fn mandelbrot_display() {
    // New image
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    print_mandelbrot(&mut img, -0.5, 0.0, 4.0); // Print fractal on img

    println!("Saving Mandelbrot."); // Save
    match img.save("mandelbrot.png") {
        Ok(s) => s,
        Err(e) => panic!("Cannot save fractal.\n{:?}", e)
    };
}

fn mandelbrot_zoom(a: f32, b: f32) {
    println!("We're zooming.\nPress CTRL + C to stop.");
    
    (0..ITER_ZOOM).into_par_iter().for_each(|i| {
        let zoom = 5.0 * (1.0 - tanh((i as f64)*0.001)) as f32;
        let mut img = RgbImage::new(WIDTH, HEIGHT);
        print_mandelbrot(&mut img, a, b, zoom); // Print fractal on img

        match img.save(format!("./render/{}.png", i)) {
            Ok(s) => s,
            Err(e) => panic!("Cannot save fractal.\n{:?}", e)
        };
    });
}

fn julia_rotation(a: f32) {
    println!("Starting render...");
    // Convert to carthesian coordinates
    (0..ITER_ROTATION).into_par_iter().for_each(|i| {
        let m: f32 = (i as f32) * 2.0 * PI/(ITER_ROTATION as f32); // Angle
        let x: f32 = f32::cos(m) * a;
        let y: f32 = f32::sin(m) * a;
        julia_display(x, y, &format!("./render/{}.png", i));
    });
}

fn julia_display(a: f32, b: f32, name: &str) {
    // New image
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    print_julia(&mut img, a, b); // Print julia fractal on img

    //println!("Saving Julia for c = {} + {}i", a, b); // Save
    match img.save(name) {
        Ok(s) => s,
        Err(e) => panic!("Cannot save fractal.\n{:?}", e)
    };
}

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 { // If no mode
        print_help();
        return
    }
    
    // Create directory
    match fs::create_dir_all("./render/") {
        Ok(s) => s,
        Err(e) => panic!("Cannot create render directory here.\n{:?}", e)
    };

    // Get mode
    let mode = &args[1];

    // Benchmark 
    let before = Instant::now();
    println!("Computing...");
    
    if mode == "-md" { // Mandelbrot
        mandelbrot_display();
    }
    else if mode == "-jr" { // Julia rotation
        // Grabbing norm
        let a: f32 = match args[2].parse::<f32>() {
            Ok(s) => s,
            Err(e) => panic!("Invalid argument [REAL PART].\n{:?}", e)
        };
        julia_rotation(a);
    }
    else if mode == "-jd" { // Julia
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
    }
    else if mode == "-mz" { // Mandelbrot zoom
        // Grabbing center point
        let a: f32 = match args[2].parse::<f32>() {
            Ok(s) => s,
            Err(e) => panic!("Invalid argument [REAL PART].\n{:?}", e)
        };
        let b: f32 = match args[3].parse::<f32>() {
            Ok(s) => s,
            Err(e) => panic!("Invalid argument [IMAGINARY PART].\n{:?}", e)
        };
        mandelbrot_zoom(a, b);
    }
    else {
        print_help();
        return
    }

    let now = Instant::now(); // Finish benchmark and print
    println!("Done ({:?})", now.duration_since(before));
}


