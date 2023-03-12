use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 640.0;

struct Snake {
    direction: KeyboardKey,
    parts: Vec<Vector2>,
    size: Vector2,
    color: Color,
}

impl Snake {
    fn set_move_dir(&mut self, direction: KeyboardKey) {
        self.direction = direction;
    }
    fn draw_snake(&mut self, draw_handle: &mut raylib::prelude::RaylibDrawHandle) {
        for part in &self.parts {
            draw_handle.draw_rectangle_v(part, self.size, self.color);
        }
    }
    fn move_snake(&mut self, draw_handle: &mut raylib::prelude::RaylibDrawHandle) {
        if self.direction == KEY_RIGHT {
            if self.parts[0].x < SCREEN_WIDTH - self.size.x {
                self.parts.insert(0, Vector2::new(self.parts[0].x + self.size.x, self.parts[0].y));
                self.parts.pop();
            }
        }
        if self.direction == KEY_LEFT {
            if self.parts[0].x >= self.size.x {
                self.parts.insert(0, Vector2::new(self.parts[0].x - self.size.x, self.parts[0].y));
                self.parts.pop();
            }
        }
        if self.direction == KEY_UP {
            if self.parts[0].y > self.size.y {
                self.parts.insert(0, Vector2::new(self.parts[0].x, self.parts[0].y - self.size.y));
                self.parts.pop();
            }
        }
        if self.direction == KEY_DOWN {
            if self.parts[0].y < SCREEN_HEIGHT - self.size.y {
                self.parts.insert(0, Vector2::new(self.parts[0].x, self.parts[0].y + self.size.y));
                self.parts.pop();
            }
        }
    }
    fn elongate_snake(&mut self) {
        self.parts.push(Vector2::new(self.parts[0].x, self.parts[0].y));
    }
}


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello, World")
        .build();
    let monitor = raylib::core::window::get_current_monitor();
    let refresh_rate = raylib::core::window::get_monitor_refresh_rate(monitor);
    rl.set_target_fps(refresh_rate as u32);

    let mut snake = Snake {
        direction: KeyboardKey::KEY_RIGHT,
        parts: vec![Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0)],
        size: Vector2::new(40.0, 40.0),
        color: Color::WHITE,
    };

    let mut noframe: f32 = 0.0;
    let mut elongate: f32 = 0.0;
    while !rl.window_should_close() {
        let dt: f32 = rl.get_frame_time();
        let interval: f32 = 0.1;
        let mut move_interval: f32 = 0.0;
        let mut d = rl.begin_drawing(&thread);
        if noframe >= interval
        {
            elongate += 1.0;
            if elongate >= 10.0 {
                snake.elongate_snake();
                elongate = 0.0;
            }
            noframe = 0.0;
            snake.move_snake(&mut d);
        } else {
            noframe += dt;
        }

        d.clear_background(Color::BLACK);
        if d.is_key_pressed(KEY_RIGHT) {
            snake.set_move_dir(KeyboardKey::KEY_RIGHT);
        }
        if d.is_key_pressed(KEY_LEFT) {
            snake.set_move_dir(KeyboardKey::KEY_LEFT);
        }
        if d.is_key_pressed(KEY_UP) {
            snake.set_move_dir(KeyboardKey::KEY_UP);
        }
        if d.is_key_pressed(KEY_DOWN) {
            snake.set_move_dir(KeyboardKey::KEY_DOWN);
        }

        //snake.move_snake(&mut d, dt);
        snake.draw_snake(&mut d);


        //snake.set_move_dir(d.get_key_pressed());


        let hello_string = "";//"Ruszaj kulka";
        //let time = now.format("%Y-%m-%d %H:%M:%S").to_string();
        d.draw_text(&hello_string, (SCREEN_WIDTH * 0.7) as i32, 24, 20, Color::WHITE);
        //d.draw_circle_v(ball.position, ball.radius, ball.color);
        //d.draw_rectangle_v(square.position, square.size, square.color);
    }
}