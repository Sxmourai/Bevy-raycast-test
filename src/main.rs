#![feature(optimize_attribute)]

pub mod screen;
pub mod rays;

use std::io::Write;

use glam::DVec3;
use rays::Ray;
use screen::Screen;

use crate::rays::send_ray;

pub const IMG_WIDTH: u32 = 1920;
pub const IMG_HEIGHT: u32 = (16/9)*IMG_WIDTH;

fn main() {
    let mut screen = Screen::new(IMG_WIDTH, IMG_HEIGHT);
    
    let aspect_ratio = 16.0 / 9.0;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((IMG_WIDTH as f64)/IMG_HEIGHT as f64);
    let camera_center = DVec3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = DVec3::new(viewport_width, 0., 0.);
    let viewport_v = DVec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMG_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMG_HEIGHT as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
                             - DVec3::new(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    for row in 0..IMG_HEIGHT {
        for column in 0..IMG_WIDTH {
            
            let pixel_center = pixel00_loc + (column as f64 * pixel_delta_u) + (row as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let unit_direction = ray_direction.normalize();
            let a = 0.5*(unit_direction.y + 1.0);
            let mut ray_color = (1.0-a)*DVec3::new(1.0, 1.0, 1.0) + a*DVec3::new(0.5, 0.7, 1.0);
            
            if hit_sphere(DVec3::new(0.,0.,-1.), 0.5, ray) {
                ray_color.x = 255.;
                ray_color.y = 0.;
                ray_color.z = 0.;
            }
            
            screen.img[((row*IMG_WIDTH+column)*3+0) as usize] = (ray_color.x*255.) as _;
            screen.img[((row*IMG_WIDTH+column)*3+1) as usize] = (ray_color.y*255.) as _;
            screen.img[((row*IMG_WIDTH+column)*3+2) as usize] = (ray_color.z*255.) as _;
        }
        print!("Line: {}/{IMG_HEIGHT}...\r", row+1);
        std::io::stdout().flush().unwrap();
    }
    screen.save();
}

fn hit_sphere(circle_center: DVec3, radius: f64, ray: Ray) -> bool {
        let oc = ray.origin - circle_center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - radius*radius;
        let discriminant = b*b - 4.*a*c;
        discriminant >= 0.
}
