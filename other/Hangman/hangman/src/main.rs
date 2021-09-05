mod graphics;
mod input;
mod words;

const MAX_GUESSES: u8 = 6;

fn main() {
    let word = words::pick_a_word();
    let mut display = "_".repeat(word.len()).chars().collect();
    let mut guesses: Vec<char> = vec![];
    let mut incorrect_guesses: u8 = 0;

    show_display(&display, &guesses, &incorrect_guesses);

    loop {
        let guess = input::get_player_input().unwrap();
        guesses.push(guess);

        let mut is_match = false;
        for (index, letter) in word.chars().enumerate() {
            if letter == guess {
                is_match = true;
                display[index] = guess;
            }
        }
        if !is_match {
            incorrect_guesses += 1;
        }

        show_display(&display, &guesses, &incorrect_guesses);

        if victory(word, &guesses) {
            break;
        }

        if game_over(incorrect_guesses as u8) {
            println!("\nVocê perdeu! A palavra era {}.\n", word);
            break;
        }
    }
}

fn show_display(display: &Vec<char>, guesses: &Vec<char>, incorrect_guesses: &u8) {
    println!("\n\n\n*******************\n\n\n");

    print!("{}   ", graphics::HANGMAN[*incorrect_guesses as usize]);

    for i in 0..display.len() {
        print!("{} ", display[i]);
    }

    print!("\n\nLetras já usadas: ");
    for i in 0..guesses.len() {
        print!("{} ", guesses[i]);
    }
}

fn victory(word: &str, guesses: &Vec<char>) -> bool {
    if word.len() != guesses.len() {
        return false;
    }

    for letter in word.chars() {
        if !guesses.contains(&letter) {
            return false;
        }
    }
    println!("Você venceu, parabéns!");
    true
}

fn game_over(incorrect_guesses: u8) -> bool {
    incorrect_guesses == MAX_GUESSES
}
