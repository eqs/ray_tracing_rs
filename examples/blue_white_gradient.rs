use ray_tracing_utils::math::{Vec3, Point3, Color, Ray};
use ray_tracing_utils::color::write_pixel;


fn ray_color(ray: Ray) -> Color {
    let unit_direction: Vec3 = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 512;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let u = j as f32 / (image_width - 1) as f32;
            let v = i as f32 / (image_height - 1) as f32;
            let ray = Ray {
                origin: origin,
                direction: lower_left_corner + horizontal*u + vertical*v - origin
            };
            let color = ray_color(ray);
            write_pixel(color);
        }
    }
}
