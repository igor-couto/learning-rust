extern crate rand;

use rand::Rng;
use std::io;

const NOT_GUESSED_YET: char = '_';
const MAX_GUESSES: u8 = 6;

fn main() {

    let stdin = io::stdin();
    let mut guesses: u8 = 0;    

    // TODO - ler a partir do arquivo

    let words: Vec<&str> = "ant baboon badger bat bear beaver camel cat clam cobra cougar
        coyote crow deer dog donkey duck eagle ferret fox frog goat 
        goose hawk lion lizard llama mole monkey moose mouse mule newt 
        otter owl panda parrot pigeon python rabbit ram rat raven 
        rhino salmon seal shark sheep skunk sloth snake spider 
        stork swan tiger toad trout turkey turtle weasel whale wolf 
        wombat zebra".split_whitespace().collect();

    let word = words[rand::thread_rng()
        .gen_range(0..words.len())]
        .to_string();

    let word_chars: Vec<char> = word.chars().collect();

    let mut display = word_chars.clone();

    for i in 0..word_chars.len() {
        display[i] = NOT_GUESSED_YET;
    }

    loop {

        if guesses == MAX_GUESSES {
            print!("Você perdeu! :(\nA palavra era {})", word);
            break;
        }

        print!("Dê um palpite: ");
        let mut guess = get_input(stdin);
        guesses = guesses + 1;

        show_display(display);
        
        if true {
            break;
        }
    }
}

fn show_display(display: Vec<char>) {
    
    for i in 0..display.len() {
        print!("{} ", display[i]);
    }

    println!("Letras já usadas: ");
}

fn get_input(stdin: io::Stdin) -> io::Result<char> {
    Ok('a')
    // let mut buffer = String::new();
    // stdin.read_line(&mut buffer)?;

    // Ok((buffer.to_owned().chars().collect())[0])

}