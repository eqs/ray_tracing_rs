use crate::math::Color;

pub fn write_pixel(color: Color) {
    let r = (color.x * 255.999) as i32;
    let g = (color.y * 255.999) as i32;
    let b = (color.z * 255.999) as i32;
    println!("{} {} {}", r, g, b);
}
