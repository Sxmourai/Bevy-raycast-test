pub struct Screen {
    pub img: Vec<u8>,
    width: u32,
    height: u32,
}
impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            img: vec!(0u8; (width*height*3) as usize),
            width,
            height,
        }
    }
    pub fn save(&self) {
        let path = std::path::Path::new(r"raycasted.png");
        let file = std::fs::File::create(path).unwrap();

        let mut encoder = png::Encoder::new(file, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.img).unwrap();
    }
}