use macroquad::prelude::*;

const ROWS: usize = 20;
const COLS: usize = 10;
const CELL_SIZE: f32 = 22.;
const PADDING: usize = (CELL_SIZE * 6.) as usize;
const SCREEN_WIDTH: i32 = PADDING as i32 + COLS as i32 * CELL_SIZE as i32 + PADDING as i32;
const SCREEN_HEIGHT: i32 = PADDING as i32 + ROWS as i32 * CELL_SIZE as i32;
const STD_FONT_SIZE: f32 = 32.;

type Board = [[Option<Color>; COLS]; ROWS];
const CYAN: Color = Color::new(0., 1., 1., 1.);

fn empty_board() -> Board {
    [[None; COLS]; ROWS]
}

const ENUM_SIZE: usize = 7;
#[derive(Clone, Copy)]
enum PieceKind {
    O,
    Z,
    T,
    S,
    I,
    J,
    L,
}

impl PieceKind {
    fn color(&self) -> Color {
        match self {
            PieceKind::O => YELLOW,
            PieceKind::Z => RED,
            PieceKind::T => PURPLE,
            PieceKind::S => GREEN,
            PieceKind::I => CYAN,
            PieceKind::J => BLUE,
            PieceKind::L => ORANGE,
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
        Self {
            kind,
            rotation: 0,
            x: (COLS as i32 / 2) - 1,
            y: 0,
        }
    }

    fn blocks_o(&self) -> [(i32, i32); 4] {
        let base_x = self.x;
        let base_y = self.y;
        [
            (base_x, base_y),
            (base_x + 1, base_y),
            (base_x, base_y + 1),
            (base_x + 1, base_y + 1),
        ]
    }

    fn blocks_z(&self) -> [(i32, i32); 4] {
        let x = self.x;
        let y = self.y;
        match self.rotation {
            0 => [(x, y), (x + 1, y), (x + 1, y + 1), (x + 2, y + 1)],
            1 => [(x + 1, y), (x, y + 1), (x + 1, y + 1), (x, y + 2)],
            2 => [(x, y), (x + 1, y), (x + 1, y + 1), (x + 2, y + 1)],
            _ => [(x + 1, y), (x, y + 1), (x + 1, y + 1), (x, y + 2)],
        }
    }

    fn blocks_t(&self) -> [(i32, i32); 4] {
        let x = self.x;
        let y = self.y;

        match self.rotation {
            0 => [(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1)],
            1 => [(x + 1, y), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)],
            2 => [(x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)],
            _ => [(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 1, y + 2)],
        }
    }

    fn blocks_s(&self) -> [(i32, i32); 4] {
        let x = self.x;
        let y = self.y;
        match self.rotation {
            0 => [(x + 1, y), (x + 2, y), (x, y + 1), (x + 1, y + 1)],
            1 => [(x, y), (x, y + 1), (x + 1, y + 1), (x + 1, y + 2)],
            2 => [(x + 1, y), (x + 2, y), (x, y + 1), (x + 1, y + 1)],
            _ => [(x, y), (x, y + 1), (x + 1, y + 1), (x + 1, y + 2)],
        }
    }

    fn blocks_i(&self) -> [(i32, i32); 4] {
        let x = self.x;
        let y = self.y;
        match self.rotation {
            0 => [(x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 3, y + 1)],
            1 => [(x + 2, y), (x + 2, y + 1), (x + 2, y + 2), (x + 2, y + 3)],
            2 => [(x, y + 2), (x + 1, y + 2), (x + 2, y + 2), (x + 3, y + 2)],
            _ => [(x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x + 1, y + 3)],
        }
    }

    fn blocks_j(&self) -> [(i32, i32); 4] {
        let x = self.x;
        let y = self.y;
        match self.rotation {
            0 => [(x, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1)],
            1 => [(x + 1, y), (x + 2, y), (x + 1, y + 1), (x + 1, y + 2)],
            2 => [(x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 2, y + 2)],
            _ => [(x + 1, y), (x + 1, y + 1), (x, y + 2), (x + 1, y + 2)],
        }
    }

    fn blocks_l(&self) -> [(i32, i32); 4] {
        let x = self.x;
        let y = self.y;
        match self.rotation {
            0 => [(x + 2, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1)],
            1 => [(x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x + 2, y + 2)],
            2 => [(x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x, y + 2)],
            _ => [(x, y), (x + 1, y), (x + 1, y + 1), (x + 1, y + 2)],
        }
    }

    fn blocks(&self) -> [(i32, i32); 4] {
        match self.kind {
            PieceKind::O => self.blocks_o(),
            PieceKind::Z => self.blocks_z(),
            PieceKind::T => self.blocks_t(),
            PieceKind::S => self.blocks_s(),
            PieceKind::I => self.blocks_i(),
            PieceKind::J => self.blocks_j(),
            PieceKind::L => self.blocks_l(),
        }
    }

    fn fall(&mut self, fall_timer: &mut f32, fall_interval: f32, dt: f32) {
        *fall_timer += dt;
        if *fall_timer >= fall_interval {
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
            PieceKind::Z => {
                for (x, y) in self.blocks() {
                    let (draw_x, draw_y) = Game::grid_to_screen_coords(y as usize, x as usize);
                    draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, self.kind.color());
                }
            }
            PieceKind::T => {
                for (x, y) in self.blocks() {
                    let (draw_x, draw_y) = Game::grid_to_screen_coords(y as usize, x as usize);
                    draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, self.kind.color());
                }
            }
            PieceKind::S => {
                for (x, y) in self.blocks() {
                    let (draw_x, draw_y) = Game::grid_to_screen_coords(y as usize, x as usize);
                    draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, self.kind.color());
                }
            }
            PieceKind::I => {
                for (x, y) in self.blocks() {
                    let (draw_x, draw_y) = Game::grid_to_screen_coords(y as usize, x as usize);
                    draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, self.kind.color());
                }
            }
            PieceKind::J => {
                for (x, y) in self.blocks() {
                    let (draw_x, draw_y) = Game::grid_to_screen_coords(y as usize, x as usize);
                    draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, self.kind.color());
                }
            }
            PieceKind::L => {
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
            PieceKind::Z => {
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
            PieceKind::T => {
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

            PieceKind::S => {
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
            PieceKind::I => {
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
            PieceKind::J => {
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
            PieceKind::L => {
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
            if y >= 0 && y < ROWS as i32 && x >= 0 && x < COLS as i32 {
                board[y as usize][x as usize] = Some(self.kind.color());
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Input {
    horiz: i8,
    rotate: bool,
    soft_drop: bool,
    hard_drop: bool,
    quit: bool,
}

#[derive(Default)]
struct KeyRepeat {
    dir: i8,
    held_time: f32,
    step_time: f32,
}

struct Game {
    board: Board,
    active: ActivePiece,
    projection: ActivePiece,
    next: PieceKind,

    fall_timer: f32,
    fall_interval: f32,

    repeat: KeyRepeat,
    das: f32,
    arr: f32,

    game_over: bool,

    graphics: DrawHandler,
}

impl Game {
    fn new() -> Self {
        let mut g = Self {
            board: empty_board(),
            active: ActivePiece::new(PieceKind::O),
            projection: ActivePiece::new(PieceKind::O),
            next: PieceKind::O,

            fall_timer: 0.0,
            fall_interval: 1.0,

            repeat: KeyRepeat::default(),
            das: 0.15,
            arr: 0.05,

            game_over: false,
            graphics: DrawHandler,
        };

        // Initialize "next" properly and spawn the first active piece
        g.next = g.random_piece();
        g.spawn_from_next();
        g.update_projection();

        g
    }

    fn random_piece(&self) -> PieceKind {
        let next: usize = macroquad::rand::gen_range(0, ENUM_SIZE);
        match next {
            0 => PieceKind::O,
            1 => PieceKind::Z,
            2 => PieceKind::T,
            3 => PieceKind::S,
            4 => PieceKind::I,
            5 => PieceKind::J,
            6 => PieceKind::L,
            _ => unreachable!(),
        }
    }

    fn spawn_from_next(&mut self) {
        self.active = ActivePiece::new(self.next);
        self.next = self.random_piece();

        if self.active.collide_check(&self.board) {
            self.game_over = true;
        }
    }

    fn read_input(&self) -> Input {
        let mut input = Input::default();

        if is_key_pressed(KeyCode::Escape) {
            input.quit = true;
        }

        if is_key_pressed(KeyCode::Space) {
            input.hard_drop = true;
        }

        if is_key_pressed(KeyCode::Up) {
            input.rotate = true; // Clockwise
        }

        input.soft_drop = is_key_down(KeyCode::Down);

        let left = is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::Right);

        input.horiz = match (left, right) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        input
    }

    fn handle_rotation(&mut self) {
        let original_rotation = self.active.rotation;
        self.active.rotation = (self.active.rotation + 1) % 4;

        if self.active.collide_check(&self.board) {
            let kicks = [(-1, 0), (1, 0), (0, -1)];
            for (dx, dy) in kicks.iter() {
                self.active.x += dx;
                self.active.y += dy;
                if !self.active.collide_check(&self.board) {
                    return;
                }
                self.active.x -= dx;
                self.active.y -= dy;
            }

            self.active.rotation = original_rotation;
        }
    }

    fn apply_input(&mut self, input: Input, dt: f32) {
        if input.quit {
            std::process::exit(0);
        }

        if input.hard_drop {
            self.active = self.projection;
            self.active.lock_piece(&mut self.board);
            self.check_line_clears();
            self.spawn_from_next();
            return;
        }

        if input.rotate {
            self.handle_rotation();
        }

        self.apply_horizontal(input.horiz, dt);

        let fall_interval = if input.soft_drop {
            self.fall_interval / 10.0
        } else {
            self.fall_interval
        };

        self.active.fall(&mut self.fall_timer, fall_interval, dt);
    }

    fn apply_horizontal(&mut self, dir: i8, dt: f32) {
        if dir == 0 {
            self.repeat = KeyRepeat::default();
            return;
        }

        if dir != self.repeat.dir {
            self.repeat.dir = dir;
            self.repeat.held_time = 0.0;
            self.repeat.step_time = 0.0;

            self.try_move_x(dir as i32);
            return;
        }

        self.repeat.held_time += dt;

        if self.repeat.held_time < self.das {
            return;
        }

        self.repeat.step_time += dt;
        while self.repeat.step_time >= self.arr {
            self.try_move_x(dir as i32);
            self.repeat.step_time -= self.arr;
        }
    }

    fn try_move_x(&mut self, dx: i32) {
        self.active.x += dx;
        if self.active.collide_check(&self.board) {
            self.active.x -= dx;
        }
    }

    fn grid_to_screen_coords(row: usize, col: usize) -> (f32, f32) {
        let x = PADDING as f32 + col as f32 * CELL_SIZE;
        let y = PADDING as f32 + row as f32 * CELL_SIZE;
        (x, y)
    }

    fn update_projection(&mut self) {
        self.projection = self.active;
        while !self.projection.collide_check(&self.board) {
            self.projection.y += 1;
        }
        self.projection.y -= 1;
    }

    fn check_line_clears(&mut self) {
        let mut new_board = empty_board();
        let mut new_row = ROWS - 1;

        for row in (0..ROWS).rev() {
            if self.board[row].iter().all(|cell| cell.is_some()) {
                continue;
            }

            new_board[new_row] = self.board[row];
            if new_row > 0 {
                new_row -= 1;
            }
        }

        self.board = new_board;
    }

    fn check_piece_lock(&mut self) {
        if self.active.collide_check(&self.board) {
            self.active.y -= 1;

            self.active.lock_piece(&mut self.board);
            self.check_line_clears();
            self.spawn_from_next();
        }
    }

    fn event_handler(&mut self) {
        let dt = get_frame_time();
        let input = self.read_input();

        self.update_projection();

        self.apply_input(input, dt);

        self.check_piece_lock();

        self.update_projection();
    }

    fn draw(&self) {
        self.graphics
            .draw(&self.board, &self.active, &self.projection);
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

    pub fn draw(&self, board: &Board, active: &ActivePiece, projection: &ActivePiece) {
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
    for c in 0..COLS {
        game.board[ROWS - 1][c] = Some(RED);
    }

    loop {
        clear_background(BLACK);

        if !game.game_over {
            game.event_handler();

            game.draw();
        } else {
            DrawHandler::draw_text_centered(
                "Game Over",
                SCREEN_HEIGHT as f32 / 2.0,
                STD_FONT_SIZE * 2.0,
                RED,
            );
            DrawHandler::draw_text_centered(
                "Press Space to restart",
                SCREEN_HEIGHT as f32 / 2.0 + 60.0,
                STD_FONT_SIZE,
                WHITE,
            );
            if is_key_released(KeyCode::Space) {
                game = Game::new();
            }
        }

        next_frame().await
    }
}
