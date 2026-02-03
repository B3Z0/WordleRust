use macroquad::prelude::*;

const ROWS: usize = 20;
const COLS: usize = 10;
const PADDING: usize = (CELL_SIZE * 6.) as usize;
const SCREEN_WIDTH: i32 = PADDING as i32 + COLS as i32 * CELL_SIZE as i32 + PADDING as i32;
const SCREEN_HEIGHT: i32 = PADDING as i32 + ROWS as i32 * CELL_SIZE as i32;
const STD_FONT_SIZE: f32 = 32.;
const CELL_SIZE:f32 = 30.;

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

type Board = [[Option<Color>; COLS]; ROWS];

fn empty_board() -> Board {
    [[None; COLS]; ROWS]
}


struct Game {
    board: Board,
    // active: ActivePiece,
    // next: PieceKind,
    
    fall_timer: f32,
    fall_interval: f32,

    game_over: bool,
}

struct DrawHandler;

impl DrawHandler {
    fn draw_text_centered(text: &str, y: f32, font_size: f32, color: Color) {
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        let x = (SCREEN_WIDTH as f32 - text_dimensions.width) / 2.0;
        draw_text(text, x, y, font_size, color);
    }

    fn draw_grid() {
        for row in 0..=ROWS {
            let y = PADDING as f32 + row as f32 * CELL_SIZE;
            draw_line(
                PADDING as f32,
                y,
                PADDING as f32 + COLS as f32 * CELL_SIZE,
                y,
                1.0,
                GRAY,
            );
        }

        for col in 0..=COLS {
            let x = PADDING as f32 + col as f32 * CELL_SIZE;
            draw_line(
                x,
                PADDING as f32,
                x,
                PADDING as f32 + ROWS as f32 * CELL_SIZE,
                1.0,
                GRAY,
            );
        }
    }

    fn draw_next_piece() {
        //square where a next piece will be shown
        let square_size = CELL_SIZE * 4.;
        let x = SCREEN_WIDTH as f32 - (PADDING as f32 + square_size) / 2.;
        let y = PADDING as f32;

        draw_rectangle(x, y, square_size, square_size * 3., DARKGRAY);
        draw_line(x, y + square_size, x + square_size, y + square_size, 1.0, GRAY);
        draw_line(
            x, y + square_size * 2.,
            x + square_size, y + square_size * 2.,
            1.0,
            GRAY,
        );
        
        draw_text("Next:", x, y - 10., STD_FONT_SIZE, WHITE);
    }

    pub fn draw_manager() {
        Self::draw_text_centered("Vlad's Tetris", 50.0, STD_FONT_SIZE, WHITE);

        Self::draw_grid();

        Self::draw_next_piece();
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
    // let mut game = Game::new();
    loop {
        clear_background(BLACK);

        event_handler();

        DrawHandler::draw_manager();
        
        next_frame().await
    }
}
