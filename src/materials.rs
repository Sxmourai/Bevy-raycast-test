use glam::DVec3;

use crate::{near_zero, objects::HitRecord, random_in_unit_sphere, rays::Ray, reflect};

#[derive(Clone)]
pub enum Material {
    Lambertian(DVec3),
    Metal(DVec3),
}
impl Material {
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        match self {
            Material::Lambertian(albedo) => {
                let mut direction = hit_record.normal + random_in_unit_sphere().normalize();
                // Catch degenerate scatter direction
                if near_zero(direction) {
                    direction = hit_record.normal;
                }
                Some((Ray {
                    origin: hit_record.point,
                    direction,
                }, albedo.clone()))
            },
            Material::Metal(albedo) => {
                let reflected = reflect(r_in.direction.normalize(), hit_record.normal);
                Some((Ray {
                    origin: hit_record.point,
                    direction: reflected,
                }, albedo.clone()))
            },
        }
    }
}
