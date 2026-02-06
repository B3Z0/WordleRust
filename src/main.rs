use macroquad::prelude::*;

const ROWS: usize = 20;
const COLS: usize = 10;
const PADDING: usize = (CELL_SIZE * 6.) as usize;
const SCREEN_WIDTH: i32 = PADDING as i32 + COLS as i32 * CELL_SIZE as i32 + PADDING as i32;
const SCREEN_HEIGHT: i32 = PADDING as i32 + ROWS as i32 * CELL_SIZE as i32;
const STD_FONT_SIZE: f32 = 32.;
const CELL_SIZE: f32 = 22.;

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

#[derive(Clone, Copy)]
struct ActivePiece {
    kind: PieceKind,
    rotation: u8,
    x: i32,
    y: i32,
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

    fn fall(&mut self, fall_timer: &mut f32, fall_interval: f32) {
        *fall_timer += get_frame_time();
        if *fall_timer > fall_interval {
            self.y += 1;
            *fall_timer = 0.0;
        }
    }

    fn draw_as_active(&self) {
        match self.kind {
            PieceKind::O => {
                for (x, y) in self.blocks() {
                    let (draw_x, draw_y) = Game::grid_to_screen_coords(y as usize, x as usize);
                    draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, self.kind.color());
                }
            }
        }
    }

    fn draw_as_projection(&self) {
        match self.kind {
            PieceKind::O => {
                for (x, y) in self.blocks() {
                    let (draw_x, draw_y) = Game::grid_to_screen_coords(y as usize, x as usize);
                    draw_rectangle_lines(
                        draw_x,
                        draw_y,
                        CELL_SIZE,
                        CELL_SIZE,
                        2.0,
                        self.kind.color(),
                    );
                }
            }
        }
    }

    fn collide_check(&self, board: &Board) -> bool {
        for (x, y) in self.blocks() {
            if x < 0 || x >= COLS as i32 || y < 0 || y >= ROWS as i32 {
                return true;
            }
            if board[y as usize][x as usize].is_some() {
                return true;
            }
        }
        false
    }

    fn lock_piece(&self, board: &mut Board) {
        for (x, y) in self.blocks() {
            board[y as usize][x as usize] = Some(self.kind.color());
        }
    }
}

struct Game {
    board: Board,
    active: ActivePiece,
    projection: ActivePiece,
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
            projection: ActivePiece::new(PieceKind::O),
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

    fn key_events(&mut self) {
        // let keys_pressed = Vec::new();

        // for key in keys_pressed {
        //     match key {
        //         KeyCode::Escape => {
        //             std::process::exit(0);
        //         }
        //         KeyCode::Right => {
        //             self.active.x += 1;
        //             if self.active.collide_check(&self.board) {
        //                 self.active.x -= 1;
        //             }
        //         }
        //         KeyCode::Left => {
        //             self.active.x -= 1;
        //             if self.active.collide_check(&self.board) {
        //                 self.active.x += 1;
        //             }
        //         }
        //         KeyCode::Down => {
        //             self.active.y += 1;
        //             if self.active.collide_check(&self.board) {
        //                 self.active.y -= 1;
        //             }
        //         }
        //         KeyCode::Space => {
        //             self.active = self.projection;
        //             self.active.lock_piece(&mut self.board);
        //             self.new_active_piece();
        //         }
        //         _ => {}
        //     }
        // }
        if is_key_down(KeyCode::Right) {
            self.active.x += 1;
            if self.active.collide_check(&self.board) {
                self.active.x -= 1;
            }
        }
        if is_key_down(KeyCode::Left) {
            self.active.x -= 1;
            if self.active.collide_check(&self.board) {
                self.active.x += 1;
            }
        }
        if is_key_down(KeyCode::Down) {
            self.active.y += 1;
            if self.active.collide_check(&self.board) {
                self.active.y -= 1;
            }
        }
        if is_key_pressed(KeyCode::Space) {
            self.active = self.projection;
            self.active.lock_piece(&mut self.board);
            self.new_active_piece();
        }
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    }

    fn new_active_piece(&mut self) {
        self.active = ActivePiece::new(self.next);
        self.next = PieceKind::O;
    }

    fn update_projection(&mut self) {
        self.projection = self.active;
        while !self.projection.collide_check(&self.board) {
            self.projection.y += 1;
        }
        self.projection.y -= 1;
    }

    fn check_piece_lock(&mut self) {
        if self.active.collide_check(&self.board) {
            self.active.y -= 1;

            self.active.lock_piece(&mut self.board);

            self.new_active_piece();
        }
    }

    fn active_event_handler(&mut self) {
        self.update_projection();

        self.active.fall(&mut self.fall_timer, self.fall_interval);

        self.check_piece_lock();
    }

    fn event_handler(&mut self) {
        self.key_events();

        self.active_event_handler();
    }

    fn draw_manager(&self) {
        self.graphics
            .draw_manager(&self.board, &self.active, &self.projection);
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
        active.draw_as_active();
    }

    fn draw_projection(&self, projection: &ActivePiece) {
        projection.draw_as_projection();
    }

    pub fn draw_manager(&self, board: &Board, active: &ActivePiece, projection: &ActivePiece) {
        Self::draw_text_centered("Vlad's Tetris", 50.0, STD_FONT_SIZE, WHITE);

        self.draw_grid();

        self.draw_next_piece();

        self.draw_board(&board);

        self.draw_active_piece(&active);

        self.draw_projection(&projection);
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
    while !game.game_over {
        clear_background(BLACK);

        game.event_handler();

        game.draw_manager();

        next_frame().await
    }
}
