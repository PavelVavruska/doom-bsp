use super::vec2d::Vec2d;

#[derive(Clone, PartialEq)]
pub struct Line {
    pub first: Vec2d,
    pub second: Vec2d,
    pub normal: Vec2d,
    pub formula_m: f64,
    pub formula_b: f64,
    pub is_portal: bool,
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

pub fn calculate_x_y_line_for_x(
    diff_rot_2_y: f64,
    diff_rot_1_y: f64,
    diff_rot_2_x: f64,
    diff_rot_1_x: f64,
    x: f64,
) -> (f64, f64) {
    let mut m = 0.0;
    if diff_rot_2_x - diff_rot_1_x != 0.0 {
        m = (diff_rot_2_y - diff_rot_1_y) / (diff_rot_2_x - diff_rot_1_x); // slope equation
    }
    let b = diff_rot_2_y - m * diff_rot_2_x; // compute B: y = m*x+b
    return (x, m * x + b); // x, y for x
}

pub fn get_new_line_segment(
    first: &Vec2d,
    second: &Vec2d,
    normal: &Vec2d,
    is_portal: bool,
) -> Line {
    let first = Vec2d::new(first.x, first.y);
    let second = Vec2d::new(second.x, second.y);
    let normal = Vec2d::new(normal.x, normal.y);
    let (a, b) = get_line_formula(&first, &second);
    let formula_m = a;
    let formula_b = b;
    Line::new(first, second, normal, formula_m, formula_b, is_portal)
}

/// normal vector (visible side)
///          ^
///          |
///          |
/// First--------Second
///
impl Line {
    pub fn new(
        first: Vec2d,
        second: Vec2d,
        normal: Vec2d,
        formula_m: f64,
        formula_b: f64,
        is_portal: bool,
    ) -> Self {
        Line {
            first,
            second,
            normal,
            formula_m,
            formula_b,
            is_portal,
        }
    }

    pub fn compare_with(&self, second_line: &Line) -> bool {
        if self.first == second_line.first {
            if self.second == second_line.second {
                return true;
            }
            return false;
        } else if self.second == second_line.first {
            if self.first == second_line.second {
                return true;
            }
            return false;
        }
        return false;
    }

    pub fn compute_length(&self) -> f64 {
        f64::sqrt(
            f64::powf(self.first.x - self.second.x, 2.0)
                + f64::powf(self.first.y - self.second.y, 2.0),
        )
    }

    pub fn is_point_under(&self, point: Vec2d) -> bool {
        /*
        let point_x = point.x;
            let point_y = point.y;
            let line_y = self.formula_m * point_x + self.formula_b;
            if line_y < point_y {
                return false;
            }
            true
        */
        //if self.normal.x != 0.0 {
        if self.first.x < point.x {
            // simplified for only vertical portals
            return false;
        }
        return true;
        //}
        //return false;
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

#[cfg(test)]
mod tests {
    use super::get_new_line_segment;
    use super::Vec2d;

    #[test]
    fn test_line_comparation() {
        let line_1_point_1 = Vec2d::new(5.0, 5.0);
        let line_1_point_2 = Vec2d::new(10.0, 5.0);
        let line_1_normal = Vec2d::new(0.0, 1.0); // south

        let line_2_point_1 = Vec2d::new(5.0, 5.0);
        let line_2_point_2 = Vec2d::new(10.0, 5.0);
        let line_2_normal = Vec2d::new(0.0, 1.0); // south

        let line_2_point_2_alternative = Vec2d::new(15.0, 5.0);

        let line_1 = get_new_line_segment(&line_1_point_1, &line_1_point_2, &line_1_normal, false);
        let line_2 = get_new_line_segment(&line_2_point_1, &line_2_point_2, &line_2_normal, false);

        assert!(line_1 == line_2, "Lines are not equal.");

        let line_2 = get_new_line_segment(
            &line_2_point_1,
            &line_2_point_2_alternative,
            &line_2_normal,
            false,
        );

        assert!(line_1 != line_2, "Lines are equal.");
    }
}
