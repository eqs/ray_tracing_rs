use rand::prelude::*;
use ray_tracing_utils::math::{Vec3, Point3, Color, Ray};
use ray_tracing_utils::color::write_pixel_sample;
use ray_tracing_utils::hittable::{Sphere, Hittable, HittableList};
use ray_tracing_utils::camera::Camera;
use ray_tracing_utils::material::{Lambertian, Metal, Dielectric};


fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(&ray, 0.001, f32::INFINITY) {
        Some(rec) => {
            match rec.material.scatter(&ray, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * ray_color(&scattered, world, depth - 1)
                },
                None => Color::new(0.0, 0.0, 0.0),
            }
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
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let r = (std::f32::consts::PI / 4.0).cos();
    let hittables: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Point3::new(r, 0.0, -1.0),
            radius: r,
            material: Box::new(Lambertian { albedo: Color::new(0.0, 0.0, 1.0) }),
        }),
        Box::new(Sphere {
            center: Point3::new(-r, 0.0, -1.0),
            radius: r,
            material: Box::new(Lambertian { albedo: Color::new(1.0, 0.0, 0.0) }),
        }),
    ];
    let world = HittableList { hittables };

    // Camera
    let origin = Point3::new(0.0, 0.0, 0.0);
    let vfov = std::f32::consts::PI / 2.0;
    let focal_length = 1.0;
    let camera = Camera::new(origin, vfov, aspect_ratio, focal_length);

    // Render

    let mut rng = thread_rng();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let j_offset: f32 = rng.gen();
                let i_offset: f32 = rng.gen();
                let u = (j as f32 + j_offset) / (image_width - 1) as f32;
                let v = (i as f32 + i_offset) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
            }

            write_pixel_sample(pixel_color, samples_per_pixel);
        }
    }
}
