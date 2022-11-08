use crate::math::{Ray, Vec3, Point3};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;

        let a = ray.direction * ray.direction;
        let b = (oc * ray.direction) * 2.0;
        let c = (oc * oc) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let mut root = (-b - disc.sqrt()) / (2.0 * a);
        if root < t_min || t_max < root {
            root = (-b + disc.sqrt()) / (2.0 * a);
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = ray.direction * outward_normal < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        return Some(HitRecord { t, p, normal, front_face });
    }
}

#[derive(Default)]
pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_rec: Option<HitRecord> = None;

        for hittable in self.hittables.iter() {
            if let Some(rec) = hittable.hit(&ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                closest_rec = Some(rec);
            }
        }

        return closest_rec;
    }
}
