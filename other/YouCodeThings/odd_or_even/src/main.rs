// Ask the user for a number. Depending on whether the number is even or odd, print out an appropriate message to the user.

use std::io::{self, Write};

fn main() {
    let number = get_input().unwrap();

    println!("O número {} é {}.", 
        number,
        if number % 2 == 0 {"Par"} else {"Impar"} );
}

fn get_input() -> io::Result<i32> {
    print!("Digite um número: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<i32>().unwrap())
}
