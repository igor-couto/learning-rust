use std::error::Error;
use std::io::{self, Write};

pub fn get_player_input() -> Result<char, Box<dyn Error>> {
    let stdin = io::stdin();

    print!("\nDê um palpite: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    let character: &char = &buffer.trim_end().parse::<char>()?.to_ascii_lowercase();

    if !character.is_ascii_alphabetic() {
        return Err("O que você digitou não é uma letra válida, tente novamente".into());
    }

    Ok(character.to_owned())
}
