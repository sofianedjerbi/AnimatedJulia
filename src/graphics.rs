use image::{Rgb, RgbImage};
use palette::{Hsv, Srgb, FromColor};

use crate::algo::{MAX_ITER, 
                  get_coefficient, 
                  get_mandelbrot_coefficient};


pub const WIDTH: u32 = 1920;  // Width of the screen
pub const HEIGHT: u32 = 1080; // Height of the screen
const ZOOM: f32 = 4.5;    // Mathematical width 


/// Return the RGB color equivalent of the HSV color
///
/// # Arguments
///
/// * `hsv`: HSV color
fn hsv_to_rgb(hsv: Hsv) -> Rgb<u8> {
    let srgb: Srgb = Srgb::from_color(hsv);
    Rgb([(srgb.red * 255.0) as u8,
         (srgb.green * 255.0) as u8,
         (srgb.blue * 255.0) as u8])
}

/// Return the Rgb Julia color for a given point 
///
/// # Arguments
///
/// * `a` - Real part of the coefficient
/// * `b` - Imaginary part of the coefficient
/// * `x` - Real part of the point
/// * `y` - Imaginary part of the point
fn get_color(a: f32, b: f32, x: f32, y: f32) -> Rgb<u8> {
    let m: f32 = MAX_ITER as f32;
    let i: f32 = get_coefficient(a, b, x, y);
    let hsv: Hsv = Hsv::new(360.0 * i/m, 1.0,
                            if i == m {0.0} else {1.0});
    return hsv_to_rgb(hsv)
}

/// Return the Rgb Mandelbrot color for a given point
///
/// # Arguments
///
/// * `x` - Real part of the point
/// * `y` - Imaginary part of the point
fn get_mandelbrot_color(x: f32, y: f32) -> Rgb<u8> {
    let m: f32 = MAX_ITER as f32;
    let i: f32 = get_mandelbrot_coefficient(x, y);
    let hsv: Hsv = Hsv::new(360.0 * i/m, 1.0,
                            if i == m {0.0} else {1.0});
    return hsv_to_rgb(hsv)
}

/// Convert pixel coords into mathematical coords
///
/// # Arguments
///
/// * `i` - Pixel position on absciss axis
/// * `j` - Pixel position on ordinate axis
/// * `w` - Width of the screen
/// * `h` - Height of the screen
/// * `zoom` - Zoom level (mathematical equivalent of 1080)
fn convert_coord(i: u32, j: u32, w: f32, h: f32, zoom: f32) -> (f32, f32) {
    return (zoom * ((i as f32) - w/2.0)/w,
           (zoom/w) * ((j as f32) - h/2.0))
}

/// Set every Julia pixel of a mutable image
///
/// # Arguments
///
/// * `image` - A mutable image container for Image cargo
/// * `a` - Real part of the constant 
/// * `b` - Complex part of the constant
pub fn print_julia(image: &mut RgbImage, a: f32, b:f32) {
    let w = WIDTH as f32;  // Convert width and height to
    let h = HEIGHT as f32; // float for computations
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let (x, y) = convert_coord(i, j, w, h, ZOOM);
            image.put_pixel(i, j, get_color(a, b, x, y));
        }
    }
}

/// Set every Mandelbrot pixel of a mutable image
///
/// # Arguments
///
/// * `image` - A mutable image container for Image cargo
/// * `a` - Absciss position of the center point
/// * `b` - Ordinal position of the center point
pub fn print_mandelbrot(image: &mut RgbImage, a: f32, b: f32, zoom: f32) {
    let w = WIDTH as f32;  // Convert width and height to
    let h = HEIGHT as f32; // float for computations
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let (x, y) = convert_coord(i, j, w, h, zoom);
            image.put_pixel(i, j, get_mandelbrot_color(x+a, y+b));
        }
    }
}

