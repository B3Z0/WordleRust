use macroquad::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const STD_FONT_SIZE: f32 = 32.;

struct Player {
    x: i32,
    y: i32,
    r: f32,
    color: Color,
    health: u8
}

impl Player {
    fn new(x: i32, y: i32, r: f32, color: Color, health: u8) -> Player {
        Player {
            x: x,
            y: y,
            r: r,
            color: color,
            health: health,
        }
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
fn draw_player(player: &Player) {
    let x = player.x as f32;
    let y = player.y as f32;
    let r = player.r;
    let color = player.color;
    draw_circle(x, y, r, color);
}

#[inline]
fn draw_controller(player: &Player) {
    draw_player(player);
    draw_ui(player.health);
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

fn event_handler(player: &mut Player, last_mouse_pos: &mut (f32, f32)) {
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

}


fn window_conf() -> Conf {
    Conf {
        window_title: "Wordle".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new(200, 200, 30., YELLOW, 100);
    let mut last_mouse_pos: (f32, f32) = mouse_position();
    loop {
        clear_background(BLACK);

        // let mouse_pos: (f32, f32) = mouse_position();
        // let player_shape = "circle";
        // let player_shape_params = ( player.x, player.y, player.r, 0.);
        
        // println!("{}", collide_mouse_basic_shape(mouse_pos, player_shape, player_shape_params));
        
        // println!("b = ({}, {})", player.x, player.y);

        event_handler(&mut player, &mut last_mouse_pos);

        // println!("a = ({}, {})", player.x, player.y);

        draw_controller(&player);
        
        next_frame().await
    }

}
