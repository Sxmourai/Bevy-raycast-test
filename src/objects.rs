use glam::{DVec3, Vec3};

use crate::{materials::Material, rays::Ray, set_face_normal};

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
    fn material(&self) -> &Material;
}

pub struct Sphere {
    pub center: DVec3, 
    pub radius: f64,
    pub material: Material,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if (discriminant < 0.) {return None};
        let sqrtd = discriminant.sqrt();

        
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if (root <= tmin || tmax <= root) {
            root = (-half_b + sqrtd) / a;
            if (root <= tmin || tmax <= root) {return None}
        }


        let normal = ray.at(root);
        let point = (normal - self.center) / self.radius;
        let outward_normal = (point - self.center) / self.radius;
        let normal = set_face_normal(ray, outward_normal);
        Some(HitRecord {
            point,
            normal,
            t: root,
            mat: self.material.clone(),
        })

        // let t = if discriminant < 0. {
        //     -1.0
        // } else {
        //     (-half_b - discriminant.sqrt() ) / a
        // };
        // if t > 0. {
        //     let n = ray.at(t) - DVec3::new(0.,0.,-1.);
        //     return Some(0.5*DVec3::new(n.x+1., n.y+1., n.z+1.));
        // }
        // None
    }
    
    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub mat: Material,
}