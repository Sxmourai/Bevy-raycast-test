#![feature(optimize_attribute)]

use std::io::Write;

pub const IMG_WIDTH: u32 = 1920;
pub const IMG_HEIGHT: u32 = 1080;

fn main() {
    let mut screen = vec![0u8; (IMG_WIDTH * IMG_HEIGHT) as usize * 3];
    for row in 0..IMG_HEIGHT {
        for column in 0..IMG_WIDTH {
            let ray = send_ray(row, column);
            screen[((row*IMG_WIDTH+column)*3+0) as usize] = ray.r;
            screen[((row*IMG_WIDTH+column)*3+1) as usize] = ray.g;
            screen[((row*IMG_WIDTH+column)*3+2) as usize] = ray.b;
        }
        print!("Line: {}/{IMG_HEIGHT}...\r", row+1);
        std::io::stdout().flush().unwrap();
    }
    save(&screen)
}

struct Ray {
    r: u8,
    g: u8,
    b: u8,
}

fn send_ray(row: u32, column: u32) -> Ray {
    let mut r = Ray {
        r: 0,
        g: 0,
        b: 0,
    };
    
    r
}


#[optimize(speed)]
fn save(data: &[u8]) {
    let path = std::path::Path::new(r"raycasted.png");
    let file = std::fs::File::create(path).unwrap();

    let mut encoder = png::Encoder::new(file, IMG_WIDTH, IMG_HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap(); // Save
}
