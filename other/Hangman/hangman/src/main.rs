extern crate rand;

use rand::Rng;
use std::io::{self, Write};

mod graphics;

const MAX_GUESSES: u8 = 6;

fn main() {
    let stdin = io::stdin();

    let word = pick_a_word();
    let mut display = "_".repeat(word.len()).chars().collect();
    let mut guesses: Vec<char> = vec![];

    println!("{}", word);

    loop {
        show_display(&display, &guesses);

        let guess = get_player_input(&stdin).unwrap();
        guesses.push(guess);

        for (index, letter) in word.chars().enumerate() {
            if letter == guess {
                display[index] = guess;
            }
        }

        if victory(word, &guesses) {
            show_display(&display, &guesses);
            break;
        }

        if game_over(guesses.len() as u8) {
            println!("Você perdeu! A palavra era {}.\n", word);
            break;
        }
    }
}

fn pick_a_word() -> &'static str {
    // TODO - ler a partir do arquivo

    let words: Vec<&str> = "ant baboon badger bat bear beaver camel cat clam cobra cougar
        coyote crow deer dog donkey duck eagle ferret fox frog goat 
        goose hawk lion lizard llama mole monkey moose mouse mule newt 
        otter owl panda parrot pigeon python rabbit ram rat raven 
        rhino salmon seal shark sheep skunk sloth snake spider 
        stork swan tiger toad trout turkey turtle weasel whale wolf 
        wombat zebra"
        .split_whitespace()
        .collect();

    words[rand::thread_rng().gen_range(0..=words.len())]
}

fn get_player_input(stdin: &io::Stdin) -> io::Result<char> {
    print!("\nDê um palpite: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<char>().unwrap())
}

fn show_display(display: &Vec<char>, guesses: &Vec<char>) {
    println!("\n\n\n*******************\n\n\n");

    // TODO - Desenhar no indice correto
    print!("{}   ", graphics::HANGMAN[0]);

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

fn game_over(number_of_guesses_given: u8) -> bool {
    number_of_guesses_given == MAX_GUESSES
}
