use std::time::{self, Instant};

use macroquad::{prelude::*, rand::gen_range};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const STD_FONT_SIZE: f32 = 32.;

struct Square {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    color: Color,
}

impl Square {
    fn new(x: f32, y: f32, size: f32, color: Color, speed: f32) -> Self {
        Square {
            x: x,
            y: y,
            size: size,
            color: color,
            speed: speed,
        }
    }
}

struct Enemies {
    enemies: Vec::<Square>,
}

impl Enemies {
    fn new() -> Self {
        Enemies {
            enemies: Vec::new(),
        }
    }

    fn gen_new_enemies(&mut self) {
        let screen_w = screen_width() as i32;
        let nr_enemies = gen_range(1, 7);

        for _ in 0..nr_enemies {
            let x = gen_range(0, screen_w - 32) as f32;

            let speed = gen_range(10, 91) as f32 / 10.;

            let enemy = Square::new(x, -32., 32., RED, speed);
            self.enemies.push(enemy);
        }

    }

    #[inline]
    pub fn update(&mut self) {
        for enemy in &mut self.enemies {
            if enemy.y < screen_height() {
                enemy.y += enemy.speed;
            }
        }
        self.enemies.retain(|sq| sq.y < screen_height());
    }

    #[inline]
    pub fn draw(&self) {
        for enemy in &self.enemies {
            let x = enemy.x as f32;
            let y = enemy.y as f32;
            let w = enemy.size;
            let h = enemy.size;
            let color = enemy.color;
            draw_rectangle(x, y, w, h, color);
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    r: f32,
    color: Color,
    health: u8
}

impl Player {
    fn new(x: i32, y: i32, r: f32, color: Color, health: u8) -> Self {
        Player {
            x: x,
            y: y,
            r: r,
            color: color,
            health: health,
        }
    }


    #[inline]
    fn draw(&self) {
        let x = self.x as f32;
        let y = self.y as f32;
        let r = self.r;
        let color = self.color;
        draw_circle(x, y, r, color);
    }
}

#[inline]
fn draw_ui(hp: u8) {
    //draw fps
    let text = &format!("FPS: {}", get_fps());
    let text_dim = measure_text(text, None, STD_FONT_SIZE as u16, 1.0);
    let (x, y) = (0., text_dim.height);
    let (x, y) = (x + 10.0, y + 10.0);
    draw_text(text, x, y, STD_FONT_SIZE, crate::WHITE);


    // draw hp
    let text = &format!("HP: {}", hp);
    let text_dim: TextDimensions = measure_text(text, None, STD_FONT_SIZE as u16, 1.0);
    let (x, y) = (screen_width() - text_dim.width, text_dim.height);
    let (x, y ) = (x - 10.0, y + 10.0);
    draw_text(text, x, y, STD_FONT_SIZE, WHITE);
}

#[inline]
fn draw_controller(player: &Player, enemies: &Enemies) {
    player.draw();
    draw_ui(player.health);
    enemies.draw();
}

fn collide_mouse_basic_shape(mouse_pos: (f32, f32), shape_type: &str, shape_params: (i32, i32, f32, f32)) -> bool {
    match shape_type {
        "circle" => {
            // Expect shape_params to be a (f32, f32, f32) tuple: (x, y, r).
            // If it's not, crash with a clear error.
            let circle_params: (i32, i32, f32) = (shape_params.0, shape_params.1, shape_params.2);

            // circle collision
            let (circle_x, circle_y, circle_r) = circle_params;
            let dist_x = mouse_pos.0 - circle_x as f32;
            let dist_y = mouse_pos.1 - circle_y as f32;
            let distance = f32::sqrt(dist_x * dist_x + dist_y * dist_y);

            if distance <= circle_r {
                return true;
            }
            return false;
            
        }
        _ => false,
    }
}

fn event_handler(player: &mut Player, last_mouse_pos: &mut (f32, f32), enemies: &mut Enemies, now: &mut Instant) {
    //keyboard events
    let keys_pressed = get_keys_pressed();
    for key in keys_pressed {
        match key {
            KeyCode::Escape => {
                std::process::exit(0);
            }
            _ => {}
        }
    }

    //mouse events
    let mouse_pos: (f32, f32) = mouse_position();
    if is_mouse_button_down(MouseButton::Left) {
        let player_shape = "circle";
        let player_shape_params = ( player.x, player.y, player.r, 0.);
        if collide_mouse_basic_shape(mouse_pos, player_shape, player_shape_params) {
            if player.color == YELLOW {
                player.color = WHITE;
                // println!("should print once!");
            }
        }
    } else {
        if player.color == WHITE {
            player.color = YELLOW;
            // println!("on release")
        }
    }

    if player.color == WHITE {
        // println!("pressed!");
        let coord_diff:(f32, f32) = (mouse_pos.0 - last_mouse_pos.0, mouse_pos.1 - last_mouse_pos.1);
        
        // println!("{:?}", coord_diff);
        // println!("{}", coord_diff.0 as i32);

        player.x += coord_diff.0 as i32;
        player.y += coord_diff.1 as i32;
    }

    *last_mouse_pos = mouse_pos;

    enemies.update();

    let elapsed = std::time::Instant::now() - *now;
    if elapsed.as_millis() >= 1800 { //1000 == 1 sec
        enemies.gen_new_enemies();
        *now += elapsed;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Falling suqares".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new(200, 200, 30., YELLOW, 100);
    let mut last_mouse_pos = mouse_position();
    let mut enemies = Enemies::new();

    let mut now = std::time::Instant::now();

    loop {
        clear_background(BLACK);

        // let mouse_pos: (f32, f32) = mouse_position();
        // let player_shape = "circle";
        // let player_shape_params = ( player.x, player.y, player.r, 0.);
        
        // println!("{}", collide_mouse_basic_shape(mouse_pos, player_shape, player_shape_params));
        
        // println!("b = ({}, {})", player.x, player.y);

        event_handler(&mut player, &mut last_mouse_pos, &mut enemies, &mut now);

        // println!("a = ({}, {})", player.x, player.y);

        draw_controller(&player, &enemies);
        
        next_frame().await
    }

}
