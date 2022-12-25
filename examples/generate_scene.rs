use rand::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use ray_tracing_utils::math::{Vec3, Point3, Color, Ray};
use ray_tracing_utils::color::write_pixel_sample;
use ray_tracing_utils::hittable::{Sphere, Hittable, HittableList};
use ray_tracing_utils::camera::Camera;
use ray_tracing_utils::material::{Lambertian, Metal, Dielectric};

fn random_scene() -> HittableList {

    let mut hittables: Vec<Box<dyn Hittable>> = vec![];
    hittables.push(
        Box::new(Sphere {
            center: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5)
            }),
        })
    );

    for i in -11..=11 {
        for j in -11..=11 {
            let choose_material: f32 = rand::random();
            let center: Point3 = Point3::new(
                i as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                j as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random();
                    let material = Lambertian { albedo };
                    hittables.push(
                        Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Box::new(material),
                        })
                    );
                } else if choose_material < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz: f32 = rand::thread_rng().gen_range(0.0..0.5);
                    let material = Metal { albedo, fuzz };
                    hittables.push(
                        Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Box::new(material),
                        })
                    );
                } else {
                    let material = Dielectric { ref_idx: 1.5 };
                    hittables.push(
                        Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Box::new(material),
                        })
                    );

                }
            }
        }
    }

    hittables.push(
        Box::new(Sphere {
            center: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(Dielectric {
                ref_idx: 1.5,
            }),
        })
    );
    hittables.push(
        Box::new(Sphere {
            center: Point3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(Lambertian {
                albedo: Color::new(0.4, 0.2, 0.1),
            }),
        })
    );
    hittables.push(
        Box::new(Sphere {
            center: Point3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(Metal {
                albedo: Color::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        })
    );

    HittableList { hittables }
}

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
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let vfov = std::f32::consts::PI / 9.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render

    let mut rng = thread_rng();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let pb = ProgressBar::new((image_height * image_width).try_into().unwrap());
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len})"
        ).unwrap()
    );

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
            pb.inc(1);
        }
    }

    pb.finish_with_message("done");
}
