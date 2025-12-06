use macroquad::prelude::*;

const WORDS: [&str; 6] = ["apple", "cigar", "sissy", "human", "zesty", "naval"];

fn window_conf() -> Conf {
    Conf {
        window_title: "My Rust Wordle".to_string(),
        window_width: 600,
        window_height: 1000,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct LetterSquare {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub letter: char,
    pub color: Color,
}

impl LetterSquare {
    pub fn new(x: f32, y: f32, size: f32, letter: char, color: Color) -> Self {
        LetterSquare {
            x,
            y,
            size,
            letter,
            color,
        }
    }
}

#[derive(Copy, Clone)]
pub struct WordRow {
    pub square: [LetterSquare; 5],
}

impl WordRow {
    pub fn new(origin_x: f32, col_origin: f32, box_size: f32, gap: f32, screen_h: f32) -> Self {
        let origin_y = screen_h / 2.0 - 2.5 * (box_size + gap);
        let square = std::array::from_fn(|col_idx| {
            let x = origin_x + col_idx as f32 * (box_size + gap);
            let y = origin_y + col_origin * (box_size + gap);
            let rect_color = DARKGRAY;
            LetterSquare::new(x, y, box_size, '\0', rect_color)
        });

        WordRow { square: square }
    }
}

pub struct WordGame {
    pub rows: [WordRow; 6],
}

impl WordGame {
    pub fn new(screen_w: f32, screen_h: f32, box_size: f32) -> Self {
        let gap: f32 = 5.0;
        let origin_x = screen_w / 2.0 - 2.5 * (box_size + gap);
        let rows: [WordRow; 6] = std::array::from_fn(|row_idx| {
            WordRow::new(origin_x, row_idx as f32, box_size, gap, screen_h)
        });

        Self { rows: rows }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let title = "Vlad's Wordle!";
    let screen_w = screen_width();
    let screen_h = screen_height();

    let rng = rand::gen_range(0, WORDS.len());
    let secret_word: &str = WORDS[rng];

    println!("Welcome to rust worlde!");
    println!("Guess the 5-letter word. You have 6 tries.");
    println!(
        "(For testing purposes, the secret word is: {})",
        secret_word
    );

    let mut guess_count = 0;
    let mut guess: [char; 5] = ['\0'; 5];

    println!("{}", guess.len());

    let mut indx = 0;

    let box_size = screen_w / 8.0;
    let mut game: WordGame = WordGame::new(screen_w, screen_h, box_size);
    let mut win_tag = false;

    loop {
        let key: Option<KeyCode> = get_last_key_pressed();
        let key: KeyCode = match key {
            Some(k) => k,
            None => KeyCode::Unknown,
        };

        let c = key as u8 as char;

        if key != KeyCode::Unknown {
            println!("key: {:?}", key);
            println!("char: {}", c);

            match key {
                KeyCode::Backspace => {
                    if indx > 0 {
                        indx -= 1;
                        guess[indx] = '\0';
                        game.rows[guess_count].square[indx].letter = '\0';
                        println!("indx = {} and word = {:?}", indx, guess);
                    }
                }
                KeyCode::Enter => {
                    if win_tag {
                        break;
                    }
                    if indx == guess.len() {
                        for i in 0..5 {
                            if guess[i] == secret_word.chars().nth(i).unwrap() {
                                game.rows[guess_count].square[i].color = GREEN;
                            } else if secret_word.contains(guess[i]) {
                                game.rows[guess_count].square[i].color = YELLOW;
                            } else {
                                game.rows[guess_count].square[i].color = DARKGRAY;
                            }
                        }

                        if guess == secret_word.chars().collect::<Vec<char>>().as_slice() {
                            println!("Congratulations you won the game!");
                            win_tag = true;
                        } else {
                            guess_count += 1;
                            indx = 0;
                            guess = ['\0'; 5];
                            if guess_count == 6 {
                                println!(
                                    "Sorry, you've used all your tries. The word was: {}",
                                    secret_word
                                );
                                break;
                            }
                        }
                    }
                }
                KeyCode::Escape => break,
                _ => (),
            };

            if c.is_alphabetic() {
                if indx < 5 {
                    guess[indx] = c.to_ascii_lowercase();
                    println!("indx = {} and word = {:?}", indx, guess);
                    indx += 1;
                    game.rows[guess_count].square[indx - 1].letter = c.to_ascii_lowercase();
                }
            }
        }

        // ---------------- Drawing part ----------------

        clear_background(BLACK);

        for row in game.rows.iter() {
            for square in row.square.iter() {
                draw_rectangle(square.x, square.y, square.size, square.size, square.color);
                if square.letter != '\0' {
                    draw_text(
                        &square.letter.to_string(),
                        square.x + square.size / 2.0
                            - measure_text(&square.letter.to_string(), None, square.size as u16, 1.0).width / 2.0,
                        square.y + square.size * 0.75,
                        square.size / 1.5,
                        WHITE,
                    );
                }
            }
        }

        draw_text(
            title,
            screen_w / 2.0 - measure_text(title, None, 50, 1.0).width / 2.0,
            screen_h / 5.0,
            50.0,
            WHITE,
        );

        if win_tag {
            let win_msg = "You Win!";
            draw_text(
                win_msg,
                screen_w / 2.0 - measure_text(win_msg, None, 40, 1.0).width / 2.0,
                screen_h - 100.0,
                40.0,
                WHITE,
            );
        }

        next_frame().await
    }
}
