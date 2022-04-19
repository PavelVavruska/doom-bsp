#[derive(Clone)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Vec2d {
        Vec2d { x, y }
    }
    pub fn add_vec_2d_xy(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    pub fn add_vec_2d(&mut self, vec2d: Vec2d) {
        self.x += vec2d.x;
        self.y += vec2d.y;
    }
    pub fn get_lenght(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalize(&self) -> Vec2d {
        let length = self.get_lenght();
        if length == 0.0 {
            return Vec2d::new(0.0, 0.0);
        }

        let x = self.x / length.abs();
        let y = self.y / length.abs();
        Vec2d::new(x, y)
    }
    pub fn multiply_by_factor(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
    pub fn dot_product_with(&self, vec_1: Vec2d) -> f64 {
        self.x * vec_1.x + self.y * vec_1.y
    }
    pub fn dot_product_between(vec_1: Vec2d, vec_2: Vec2d) -> f64 {
        vec_1.x * vec_2.x + vec_1.y * vec_2.y
    }
}
