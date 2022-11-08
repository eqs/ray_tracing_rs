use crate::math::{Color, clamp};

pub fn write_pixel(color: Color) {
    let r = (clamp(color.x, 0.0, 0.999) * 256.0) as i32;
    let g = (clamp(color.y, 0.0, 0.999) * 256.0) as i32;
    let b = (clamp(color.z, 0.0, 0.999) * 256.0) as i32;
    println!("{} {} {}", r, g, b);
}

pub fn write_pixel_sample(color: Color, samples_per_pixel: i32) {
    let s = 1.0 / samples_per_pixel as f32;
    let r = (clamp(color.x * s, 0.0, 0.999) * 256.0) as i32;
    let g = (clamp(color.y * s, 0.0, 0.999) * 256.0) as i32;
    let b = (clamp(color.z * s, 0.0, 0.999) * 256.0) as i32;
    println!("{} {} {}", r, g, b);
}
