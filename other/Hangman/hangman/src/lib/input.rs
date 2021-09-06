use std::io::{self, Write};

pub fn get_player_input() -> io::Result<char> {
    let stdin = io::stdin();

    print!("\nDê um palpite: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    let character: &char = &buffer
        .trim_end()
        .parse::<char>()
        .unwrap()
        .to_ascii_lowercase();

    // TODO - Esse é um erro recuperável. Não devia fazer panic
    if !character.is_ascii_alphabetic() {
        panic!("Letra inválida.");
    }

    Ok(character.to_owned())
}
