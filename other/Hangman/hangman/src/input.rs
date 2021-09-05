use std::io::{self, Write};

pub fn get_player_input() -> io::Result<char> {
    let stdin = io::stdin();

    print!("\nDê um palpite: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<char>().unwrap())
}
