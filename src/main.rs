use macroquad::prelude::*;

const ROWS: usize = 20;
const COLS: usize = 10;
const PADDING: usize = (CELL_SIZE * 6.) as usize;
const SCREEN_WIDTH: i32 = PADDING as i32 + COLS as i32 * CELL_SIZE as i32 + PADDING as i32;
const SCREEN_HEIGHT: i32 = PADDING as i32 + ROWS as i32 * CELL_SIZE as i32;
const STD_FONT_SIZE: f32 = 32.;
const CELL_SIZE: f32 = 30.;

type Board = [[Option<Color>; COLS]; ROWS];

fn empty_board() -> Board {
    [[None; COLS]; ROWS]
}

#[derive(Clone, Copy)]
enum PieceKind {
    O,
}

impl PieceKind {
    fn color(&self) -> Color {
        match self {
            PieceKind::O => YELLOW,
        }
    }
}

impl ActivePiece {
    fn new(kind: PieceKind) -> Self {
        // Spawn near top center
        Self {
            kind,
            rotation: 0,
            x: (COLS as i32 / 2) - 1,
            y: 0,
        }
    }

    fn blocks(self) -> [(i32, i32); 4] {
        // For milestone: only O piece (2x2), rotation irrelevant
        match self.kind {
            PieceKind::O => [
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x, self.y + 1),
                (self.x + 1, self.y + 1),
            ],
        }
    }
}

#[derive(Clone, Copy)]
struct ActivePiece {
    kind: PieceKind,
    rotation: u8,
    x: i32,
    y: i32,
}

struct Game {
    board: Board,
    active: ActivePiece,
    next: PieceKind,

    fall_timer: f32,
    fall_interval: f32,

    game_over: bool,

    graphics: DrawHandler,
}

impl Game {
    fn new() -> Self {
        Self {
            board: empty_board(),
            active: ActivePiece::new(PieceKind::O),
            next: PieceKind::O,
            fall_timer: 0.0,
            fall_interval: 1.0,
            game_over: false,
            graphics: DrawHandler,
        }
    }

    fn grid_to_screen_coords(row: usize, col: usize) -> (f32, f32) {
        let x = PADDING as f32 + col as f32 * CELL_SIZE;
        let y = PADDING as f32 + row as f32 * CELL_SIZE;
        (x, y)
    }

    fn active_collide_check(&mut self) -> bool {
        println!("--");
        for (x, y) in self.active.blocks() {
            println!("Checking block at ({}, {})", x, y);
            if x < 0 || x >= COLS as i32 || y < 0 || y >= ROWS as i32 {
                println!("Collision with boundary at ({}, {})", x, y);
                return true;
            }
            if self.board[y as usize][x as usize].is_some() {
                println!("Collision with placed block at ({}, {})", x, y);
                return true;
            }
        }
        false
    }

    fn event_handler(&mut self) {
        //keyboard events
        let keys_pressed = get_keys_pressed();
        for key in keys_pressed {
            match key {
                KeyCode::Escape => {
                    std::process::exit(0);
                }
                KeyCode::Right => {
                    self.active.x += 1;
                    if self.active_collide_check() {
                        self.active.x -= 1;
                    }
                }
                KeyCode::Left => {
                    self.active.x -= 1;
                    if self.active_collide_check() {
                        self.active.x += 1;
                    }
                }
                KeyCode::Down => {
                    self.active.y += 1;
                    if self.active_collide_check() {
                        self.active.y -= 1;
                    }
                }
                _ => {}
            }
        }
    }
    fn draw_manager(&self) {
        self.graphics.draw_manager(&self.board, &self.active);
    }
}

struct DrawHandler;

impl DrawHandler {
    fn draw_text_centered(text: &str, y: f32, font_size: f32, color: Color) {
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        let x = (SCREEN_WIDTH as f32 - text_dimensions.width) / 2.0;
        draw_text(text, x, y, font_size, color);
    }

    fn draw_grid(&self) {
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

    fn draw_next_piece(&self) {
        //square where a next piece will be shown
        let square_size = CELL_SIZE * 4.;
        let x = SCREEN_WIDTH as f32 - (PADDING as f32 + square_size) / 2.;
        let y = PADDING as f32;

        draw_rectangle(x, y, square_size, square_size * 3., DARKGRAY);
        draw_line(
            x,
            y + square_size,
            x + square_size,
            y + square_size,
            1.0,
            GRAY,
        );
        draw_line(
            x,
            y + square_size * 2.,
            x + square_size,
            y + square_size * 2.,
            1.0,
            GRAY,
        );

        draw_text("Next:", x, y - 10., STD_FONT_SIZE, WHITE);
    }

    fn draw_board(&self, board: &Board) {
        //draw placed blocks
        for row in 0..ROWS {
            for col in 0..COLS {
                if let Some(color) = board[row][col] {
                    let (x, y) = Game::grid_to_screen_coords(row, col);
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
                }
            }
        }
    }

    fn draw_active_piece(&self, active: &ActivePiece) {
        for (x, y) in active.blocks() {
            let (screen_x, screen_y) = Game::grid_to_screen_coords(y as usize, x as usize);
            draw_rectangle(
                screen_x,
                screen_y,
                CELL_SIZE,
                CELL_SIZE,
                active.kind.color(),
            );
        }
    }

    pub fn draw_manager(&self, board: &Board, active: &ActivePiece) {
        Self::draw_text_centered("Vlad's Tetris", 50.0, STD_FONT_SIZE, WHITE);

        self.draw_grid();

        self.draw_next_piece();

        self.draw_board(board);

        self.draw_active_piece(active);
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
    let mut game = Game::new();
    game.board[ROWS - 1][COLS / 2] = Some(RED); //test block
    loop {
        clear_background(BLACK);

        game.event_handler();

        game.draw_manager();

        next_frame().await
    }
}
