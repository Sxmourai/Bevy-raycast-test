use glam::DVec3;

pub struct Camera {
    pub fst_pixel_loc: DVec3,
    pub pixel_delta_u: DVec3,
    pub pixel_delta_v: DVec3,
    pub center: DVec3,
}
impl Camera {
    pub fn new(width: f64, height: f64) -> Self {
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (width/height);
        let camera_center = DVec3::new(0., 0., 0.);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = DVec3::new(viewport_width, 0., 0.);
        let viewport_v = DVec3::new(0., -viewport_height, 0.);
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center
                                - DVec3::new(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / width;
        let pixel_delta_v = viewport_v / height;
        let fst_pixel_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            fst_pixel_loc,
            pixel_delta_u,
            pixel_delta_v,
            center: DVec3::new(0., 0., 0.)
        }

    }
}