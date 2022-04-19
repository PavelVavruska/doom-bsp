pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64, // RAD
    pub fov: f64,   // RAD
}

impl Player {
    pub fn new(x: f64, y: f64, angle: f64, fov: f64) -> Player {
        Player { x, y, angle, fov }
    }
}
