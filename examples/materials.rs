use ray_tracing_utils::math::{Vec3, Point3, Color, Ray};
use ray_tracing_utils::color::write_pixel;
use ray_tracing_utils::hittable::{Sphere, Hittable, HittableList};
use ray_tracing_utils::camera::Camera;


fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    match world.hit(&ray, 0.0, f32::INFINITY) {
        Some(rec) => {
            (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
        },
        None => {
            let unit_direction: Vec3 = ray.direction.normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        },
    }
}

fn main() {

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 512;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // World

    let hittables: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 }),
        Box::new(Sphere { center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 }),
    ];
    let world = HittableList { hittables };

    // Camera
    let origin = Point3::new(0.0, 0.0, 0.0);
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let camera = Camera::new(origin, viewport_height, aspect_ratio, focal_length);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let u = j as f32 / (image_width - 1) as f32;
            let v = i as f32 / (image_height - 1) as f32;
            let ray = camera.get_ray(u, v);
            let color = ray_color(&ray, &world);
            write_pixel(color);
        }
    }
}
