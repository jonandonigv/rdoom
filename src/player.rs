use sdl2::{event::Event, keyboard::Keycode};

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
    moving_up: bool,
    movind_down: bool,
    moving_right: bool,
    moving_left: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            speed: 5.0,
            moving_up: false,
            movind_down: false,
            moving_right: false,
            moving_left: false,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => match *key {
                Keycode::W => self.moving_up = true,
                Keycode::S => self.movind_down = true,
                Keycode::A => self.moving_left = true,
                Keycode::D => self.moving_right = true,
                _ => {}
            },
            Event::KeyUp {
                keycode: Some(key), ..
            } => match *key {
                Keycode::W => self.moving_up = false,
                Keycode::S => self.movind_down = false,
                Keycode::A => self.moving_left = false,
                Keycode::D => self.moving_right = false,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn update(&mut self, map: &crate::map::Map, delta_time: f32) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let move_dist = self.speed * delta_time;

        if self.moving_up {
            dy -= move_dist;
        }
        if self.movind_down {
            dy += move_dist;
        }
        if self.moving_right {
            dx += move_dist;
        }
        if self.moving_left {
            dx -= move_dist;
        }

        // Check collision with map walls
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if !map.is_wall((new_x.floor() as i32, new_y.floor() as i32)) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
