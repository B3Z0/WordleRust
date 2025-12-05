use macroquad::prelude::*;

const WORDS: [&str; 6] = [
    "apple",
    "cigar",
    "sissy",
    "human",
    "zesty",
    "naval"
    ];


#[macroquad::main("Wordle")]
async fn main() {
    let rng = rand::gen_range(0, WORDS.len());
    let secret_word: &str = WORDS[rng];

    println!("Welcome to rust worlde!");
    println!("Guess the 5-letter word. You have 6 tries.");
    println!("(For testing purposes, the secret word is: {})", secret_word);

    let mut guess: [char; 5] = ['\0'; 5];

    println!("{}", guess.len());

    let mut indx = 0;
    loop {
        clear_background(BLACK);

        let key: Option<KeyCode> = get_last_key_pressed();
        let key: KeyCode = match key {
            Some(k) => k,
            None => { next_frame().await; continue; },
        };
        
        let c = key as u8 as char;


        if key != KeyCode::Unknown {
            println!("key: {:?}", key);
            println!("char: {}", c);
        }

        let command = match key {
            KeyCode::Backspace => {
                if indx > 0 {
                    indx -= 1;
                    guess[indx] = '\0';
                    println!("indx = {} and word = {:?}", indx, guess);
                }
            },
            KeyCode::Enter => {
                if indx == guess.len() {
                    if guess == secret_word.chars().collect::<Vec<char>>().as_slice() {
                        println!("Congratulations you won the game!");
                        break;
                    }
                }
            },
            _ => {()},
        };

        if c.is_alphabetic() {
            if indx < 5 {
                guess[indx] = c.to_ascii_lowercase();
                println!("indx = {} and word = {:?}", indx, guess);
                indx += 1;
            }
        }

        next_frame().await
    }


    // for attempt in 1..=6 {
    //     print!("Attempt {}/6: ", attempt);
    //     io::stdout().flush().expect("Failed to flush stdout");

    //     let mut guess = String::new();

    // }



}
