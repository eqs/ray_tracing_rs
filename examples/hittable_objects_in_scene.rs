use ray_tracing_utils::math::{Vec3, Point3, Color, Ray};
use ray_tracing_utils::color::write_pixel;
use ray_tracing_utils::hittable::{Sphere, HitRecord, Hittable};


fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> Option<HitRecord> {
    let sphere = Sphere { center, radius };
    sphere.hit(&ray, 0.0, f32::INFINITY)

}

fn ray_color(ray: &Ray) -> Color {
    match hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        Some(record) => {
            let n = (ray.at(record.t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
            Color::new(n.x+1.0, n.y+1.0, n.z+1.0) * 0.5
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
            let color = ray_color(&ray);
            write_pixel(color);
        }
    }
}
