use crate::math::{Vec3, Ray, Color, minval};
use crate::hittable::HitRecord;
use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Default, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered: Ray = Ray::new(rec.p, scatter_direction);
        let attenuation: Color = self.albedo;

        Some((scattered, attenuation))
    }
}

#[derive(Default, Clone)]
pub struct Metal {
    pub fuzz: f32,
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray.direction.normalized(), rec.normal);
        let fuzziness = self.fuzz * Vec3::random_in_unit_sphere();
        let scattered = Ray::new(rec.p, reflected + fuzziness);
        let attenuation = self.albedo;

        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

#[derive(Default, Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray.direction.normalized();
        let cos_theta = minval(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            Some((scattered, attenuation))
        } else {

            let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
            let scattered = Ray::new(rec.p, refracted);
            Some((scattered, attenuation))
        }
    }
}
