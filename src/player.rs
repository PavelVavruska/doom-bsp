use std::f64::consts::PI;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub view_angle: f64, // RAD
    pub fov: f64,        // RAD
    pub move_angle: f64,
    pub move_speed: f64,
}

impl Player {
    pub fn new(
        x: f64,
        y: f64,
        view_angle: f64,
        fov: f64,
        move_angle: f64,
        move_speed: f64,
    ) -> Player {
        Player {
            x,
            y,
            view_angle,
            fov,
            move_angle,
            move_speed,
        }
    }

    pub fn move_forward(&mut self) {
        self.x += self.move_angle.cos() * self.move_speed;
        self.y += self.move_angle.sin() * self.move_speed;
    }

    pub fn move_backward(&mut self) {
        self.x -= self.move_angle.cos() * self.move_speed;
        self.y -= self.move_angle.sin() * self.move_speed;
    }

    pub fn turn_left(&mut self) {
        self.view_angle -= 0.5;
        self.move_angle -= 0.5;

        if self.view_angle > PI {
            self.view_angle -= 2.0 * PI;
        }
        if self.move_angle > PI {
            self.move_angle -= 2.0 * PI;
        }
        if self.view_angle < PI {
            self.view_angle += 2.0 * PI;
        }
        if self.move_angle < PI {
            self.move_angle += 2.0 * PI;
        }
    }

    pub fn turn_right(&mut self) {
        self.view_angle += 0.5;
        self.move_angle += 0.5;
        if self.view_angle > PI {
            self.view_angle -= 2.0 * PI;
        }
        if self.move_angle > PI {
            self.move_angle -= 2.0 * PI;
        }
        if self.view_angle < PI {
            self.view_angle += 2.0 * PI;
        }
        if self.move_angle < PI {
            self.move_angle += 2.0 * PI;
        }
    }
}
