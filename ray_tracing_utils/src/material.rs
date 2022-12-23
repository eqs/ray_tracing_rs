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
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered: Ray = Ray::new(rec.p, scatter_direction);
        let attenuation: Color = self.albedo;

        Some((scattered, attenuation))
    }
}
