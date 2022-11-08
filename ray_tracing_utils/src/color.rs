use crate::math::{Color, clamp};

pub fn write_pixel(color: Color) {
    let r = (clamp(color.x, 0.0, 0.999) * 256.0) as i32;
    let g = (clamp(color.y, 0.0, 0.999) * 256.0) as i32;
    let b = (clamp(color.z, 0.0, 0.999) * 256.0) as i32;
    println!("{} {} {}", r, g, b);
}
