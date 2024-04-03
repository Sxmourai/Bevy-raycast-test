use glam::{DVec2, DVec3, UVec2};
use rand::random;

use crate::{camera::Camera, rays::Ray, IMG_HEIGHT, IMG_WIDTH};

pub struct Screen {
    pub img: Vec<u8>,
    width: f64,
    height: f64,
    camera: Camera,
    pub samples_per_pixel: f64,
}
impl Screen {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            img: vec!(0u8; (width*height*3.) as usize),
            width,
            height,
            camera: Camera::new(width.into(), height.into()),
            samples_per_pixel: 2.,
        }
    }

    pub fn get_ray(&self, x:f64, y:f64) -> Ray {
        let pixel_center = self.camera.fst_pixel_loc + (x * self.camera.pixel_delta_u) + (y * self.camera.pixel_delta_v);
        let ray_direction = (pixel_center + self.pixel_sample_square()) - self.camera.center;
        Ray::new(self.camera.center, ray_direction)
    }
    fn pixel_sample_square(&self) -> DVec3 {
        let px = random::<f64>() - 0.5;
        let py = random::<f64>() - 0.5;
        (px * self.camera.pixel_delta_u) + (py * self.camera.pixel_delta_v)
    }

    pub fn write_pixel(&mut self, pos: DVec2, mut pixel_color: DVec3) {
        pixel_color *= 1. / self.samples_per_pixel;

    // dbg!(pixel_color);
        self.img[((pos.y*self.width+pos.x)*3.+0.) as usize] = (pixel_color.x.sqrt()*255.).min(255.) as _;
        self.img[((pos.y*self.width+pos.x)*3.+1.) as usize] = (pixel_color.y.sqrt()*255.).min(255.) as _;
        self.img[((pos.y*self.width+pos.x)*3.+2.) as usize] = (pixel_color.z.sqrt()*255.).min(255.) as _;

    }

    pub fn save(&self) {
        let path = std::path::Path::new(r"raycasted.png");
        let file = std::fs::File::create(path).unwrap();

        let mut encoder = png::Encoder::new(file, self.width as _, self.height as _);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        // self.process_image();
        writer.write_image_data(&self.img).unwrap();
    }
}