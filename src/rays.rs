use glam::DVec3;


pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self {
            origin,
            direction,
        }
    }
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}

#[optimize(speed)]
/// Takes x, y coords, 
pub fn send_ray(x: f64, y: f64) -> (u8, u8, u8) {
    let ray = Ray {
        origin: DVec3::new(x, y, 0.),
        direction: DVec3::ONE,
    };
    
    (0, 0, 0)
}

