use macroquad::prelude::*;

const SCREEN_WIDTH: i32 = 10 * 25 + 200;
const SCREEN_HEIGHT: i32 = 20 * 25 + 200;
const STD_FONT_SIZE: f32 = 32.;

fn event_handler() {
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
}

struct DrawHandler;

impl DrawHandler {
    fn draw_text_centered(text: &str, y: f32, font_size: f32, color: Color) {
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        let x = (SCREEN_WIDTH as f32 - text_dimensions.width) / 2.0;
        draw_text(text, x, y, font_size, color);
    }


    pub fn draw_manager() {
        Self::draw_text_centered("Vlad's Tetris", 50.0, STD_FONT_SIZE, WHITE);

        // draw_grid();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Vlad's Tetris".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    loop {
        clear_background(BLACK);

        event_handler();

        DrawHandler::draw_manager();
        
        next_frame().await
    }

}
