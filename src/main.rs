#![feature(optimize_attribute)]
#![cfg_attr(debug_assertions, allow(dead_code, unused))]


pub mod screen;
pub mod rays;
pub mod camera;
pub mod objects;
pub mod materials;

use std::io::Write;

use glam::{DVec2, DVec3, Vec3};
use materials::Material;
use objects::{Hittable, Sphere};
use rand::{distributions::Bernoulli, rngs::StdRng, Rng};
use rays::Ray;
use screen::Screen;


pub const IMG_WIDTH: u32 = 1920;
pub const IMG_HEIGHT: u32 = ((9./16.)*IMG_WIDTH as f64) as _;

#[optimize(speed)]
fn main() {
    let mut screen = Screen::new(IMG_WIDTH.into(), IMG_HEIGHT.into());
    let objects:Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {center:DVec3::new(0.,-100.5,-1.),radius:100., material: Material::Lambertian(DVec3::new(0.8, 0.8, 0.0)) }),
        Box::new(Sphere {center:DVec3::new(0., 0., -1.),radius:0.5, material: Material::Lambertian(DVec3::new(0.7, 0.3, 0.3)) }),
        Box::new(Sphere {center:DVec3::new(1., 0., -1.),radius:0.5, material: Material::Metal(DVec3::new(0.8, 0.8, 0.8)) }),
        Box::new(Sphere {center:DVec3::new(-1., 0., -1.),radius:0.5, material: Material::Metal(DVec3::new(0.8, 0.6, 0.2)) }),
    ];
    for y in 0..IMG_HEIGHT {
        for x in 0..IMG_WIDTH {
            let mut color = DVec3::ZERO;
            for sample in 0..screen.samples_per_pixel as _ {
                let ray = screen.get_ray(x.into(),y.into());
                color += ray_color(&screen, &objects, ray, 100);
            }
            screen.write_pixel(DVec2::new(x.into(),y.into()), color);
        }
        print!("Line: {}/{IMG_HEIGHT}...\r", y+1);
        std::io::stdout().flush().unwrap();
    }
    screen.save();
}
static mut RND_DISTRIB: Option<StdRng> = None;
fn rnd_f64(min: f64, max: f64) -> f64 {
    if unsafe { RND_DISTRIB.is_none() } {
        unsafe { RND_DISTRIB.replace(<rand::rngs::StdRng as rand::SeedableRng>::from_entropy()) };
    };
    rand::Rng::sample::<f64, rand::distributions::Standard>(unsafe { RND_DISTRIB.as_mut().unwrap() }, rand::distributions::Standard)
}

fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let vec = DVec3::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
        );

        if vec.length_squared() < 1. {
            break vec;
        }
    }
}

fn random_on_hemisphere(normal: DVec3) -> DVec3 {
    let on_unit_sphere = random_in_unit_sphere().normalize();
    if on_unit_sphere.dot(normal) > 0. { // In the same hemisphere as the normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

/// Return true if the vector is close to zero in all dimensions.
fn near_zero(vec: DVec3) -> bool {
    let s = 1e-8;
    vec.x<s && vec.y<s && vec.z<s
}
fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.*v.dot(n)*n
}
// Sets the hit record normal vector.
// NOTE: the parameter `outward_normal` is assumed to have unit length.
fn set_face_normal(ray: &Ray, outward_normal: DVec3) -> DVec3 {
    let front_face = ray.direction.dot(outward_normal);
    if front_face < 0. {outward_normal} else {-outward_normal}
}

fn ray_color(screen: &Screen, objects: &Vec<Box<dyn Hittable>>, ray: Ray, depth: u8) -> DVec3 {
    if depth == 0 {return DVec3::ZERO}
    let mut hit_anything = false;
    let mut closest_so_far = f64::MAX;
    let mut last_hit = None;
    for obj in objects {
        if let Some(record) = obj.hit(&ray, 0.001, closest_so_far) {
            hit_anything = true;
            closest_so_far = record.t;
            last_hit.replace(record);
            // let dir = record.normal + random_in_unit_sphere().normalize();
            // color = 0.3 * ray_color(screen, objects, Ray::new(record.point, dir), depth - 1);
        }
    }
    if hit_anything {
        let record = last_hit.unwrap();
        if let Some((scattered, attenuation)) = record.mat.scatter(&ray, &record) {
            return attenuation * ray_color(screen, objects, scattered, depth-1)
        } else {
            return DVec3::ZERO;
        }
    }
    let unit_direction = ray.direction.normalize();
    let a = 0.3*(unit_direction.y + 1.0);
    let mut color = (1.0-a)*DVec3::new(1.0, 1.0, 1.0) + a*DVec3::new(0.5, 0.7, 1.0);
    return color
}