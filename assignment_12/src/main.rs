/*
Desenvolver um algoritmo para ler o valor inteiro da idade
de uma pessoa e imprimir uma das mensagens: se a idade
é menor que 13: Criança, se maior que 13 e menor que 20: Adolescente, se a idade maior que 20
e menor que 60: Adulto e se idade maior que 60: Idoso.
*/

use std::io::{self,Write};

fn main() {
    let age = get_age().unwrap();

    if age < 13 {
        println!("Criança");
        return;
    } 

    if age < 20 {
        println!("Adolescente");
        return;
    }

    if age < 60 {
        println!("Adulto");
        return;
    }

    println!("Idoso");
}

fn get_age() -> io::Result<u8>{
    let stdin = io::stdin();
    print!("Idade: ");
    io::stdout().flush()?;
    
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<u8>().unwrap())
}