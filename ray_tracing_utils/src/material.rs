use crate::math::{Vec3, Ray, Color};
use crate::hittable::HitRecord;
use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray.direction.normalized(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
