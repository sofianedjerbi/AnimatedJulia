use image::{Rgb, RgbImage};
use palette::{Hsv, Srgb, FromColor};

use crate::julia::{MAX_ITER, get_coefficient};


pub const WIDTH: u32 = 1920;  // Width of the screen
pub const HEIGHT: u32 = 1080; // Height of the screen


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
    let i: f32 = get_coefficient(a, b, x, y) as f32;
    let hsv: Hsv = Hsv::new(360.0 * i/m, 1.0,
                            if i == m {0.0} else {1.0});
    return hsv_to_rgb(hsv)
}

///
//fn convert_coord(i: u16, j: u16) -> {

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
            let x = 4.0 * ((i as f32) - w/2.0)/w;         // + px
            let y = (4.0/w) * ((j as f32) - h/2.0);     // + py
            image.put_pixel(i, j, get_color(a, b, x, y));
        }
    }
}
