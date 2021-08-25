/*
Create a program that asks the user to enter their name and their age. Print out a message addressed to them that tells them the year that they will turn 100 years old.
*/

extern crate chrono;

use std::io;
use chrono::prelude::*;

const TARGET_YEAR: u16 = 100;

fn main() {

    println!("Qual é o seu nome?");
    let name = get_input()
        .expect("Algo deu errado, por favor tente novamente.");

    println!("Qual é a sua idade?");
    let age: u16 = get_input()
        .expect("Algo deu errado, por favor tente novamente.")
        .parse::<u16>()
        .expect("Idade inválida");

    let current_year: u16 = Utc::now().year() as u16;    
    let dif = TARGET_YEAR - age;

    if dif <= 0 {
        println!("O {} fez 100 anos em {}", name, current_year - dif );
    } 
    else {
        println!("O {} vai fazer 100 anos em {}", name, current_year + dif );
    }
}

fn get_input() -> io::Result<String> {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input)?;

    Ok(input.trim_end().to_owned())
}