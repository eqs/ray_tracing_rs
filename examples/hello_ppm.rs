use ray_tracing_utils::math::Color;
use ray_tracing_utils::color::write_pixel;

fn main() {

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let r = j as f32 / (image_width - 1) as f32;
            let g = i as f32 / (image_height - 1) as f32;
            let b = 0.25;
            let color = Color::new(r, g, b);
            write_pixel(color);
        }
    }
}
