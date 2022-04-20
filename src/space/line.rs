use super::vec2d::Vec2d;

#[derive(Clone, PartialEq)]
pub struct Line {
    pub first: Vec2d,
    pub second: Vec2d,
    pub normal: Vec2d,
    pub formula_m: f64,
    pub formula_b: f64,
}

/// returns m and b from line formula y=mx+b
pub fn get_line_formula(point1: &Vec2d, point2: &Vec2d) -> (f64, f64) {
    let x1 = point1.x;
    let y1 = point1.y;
    let x2 = point2.x;
    let y2 = point2.y;
    let delta_x = x1 - x2;
    let delta_y = y1 - y2;
    let mut m = 0.0;
    if delta_x != 0.0 {
        m = delta_y / delta_x;
    }

    // y = m*x + b
    let b = y1 - m * x1; // calculate b from one point
    (m, b)
}

pub fn get_new_line_segment(first: &Vec2d, second: &Vec2d, normal: &Vec2d) -> Line {
    let first = Vec2d::new(first.x, first.y);
    let second = Vec2d::new(second.x, second.y);
    let normal = Vec2d::new(normal.x, normal.y);
    let (a, b) = get_line_formula(&first, &second);
    let formula_m = a;
    let formula_b = b;
    Line::new(first, second, normal, formula_m, formula_b)
}

/// normal vector (visible side)
///          ^
///          |
///          |
/// First--------Second
///
impl Line {
    pub fn new(first: Vec2d, second: Vec2d, normal: Vec2d, formula_m: f64, formula_b: f64) -> Self {
        Line {
            first,
            second,
            normal,
            formula_m,
            formula_b,
        }
    }

    pub fn compute_length(&self) -> f64 {
        f64::sqrt(
            f64::powf(self.first.x - self.second.x, 2.0)
                + f64::powf(self.first.y - self.second.y, 2.0),
        )
    }

    pub fn is_point_under(&self, point: Vec2d) -> bool {
        let point_x = point.x;
        let point_y = point.y;
        let line_y = self.formula_m * point_x + self.formula_b;
        if line_y < point_y {
            return false;
        }
        true
    }

    /// scale ends
    pub fn scale_first_end(&mut self, scale: f64) {
        let x1 = self.first.x;
        let y1 = self.first.y;
        let x2 = self.second.x;
        let y2 = self.second.y;
        let delta_x = x1 - x2;
        let delta_y = y1 - y2;
        self.first.x = x2 + delta_x * scale;
        self.first.y = y2 + delta_y * scale;
    }

    /// scale ends
    pub fn scale_second_end(&mut self, scale: f64) {
        let x1 = self.first.x;
        let y1 = self.first.y;
        let x2 = self.second.x;
        let y2 = self.second.y;
        let delta_x = x1 - x2;
        let delta_y = y1 - y2;
        self.second.x = x2 + delta_x * scale;
        self.second.y = y2 + delta_y * scale;
    }
}
