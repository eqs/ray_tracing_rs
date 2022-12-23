use crate::math::{Vec3, Ray, Color};
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

        Some((scattered, attenuation))
    }
}
