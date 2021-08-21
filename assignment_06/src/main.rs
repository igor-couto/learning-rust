//Ler um número inteiro e informar se ele é divisível por 2.

use std::env;

fn main() {

    let mut args: Vec<String> = env::args().collect(); 
    args.remove(0);

    if args.len() != 1 {
        println!("Por favor, informe 1 e apenas 1 argumento. Foram encontrados {} argumentos.", args.len());
        return;
    }

    let number = match args[0].parse::<i32>() {
        Ok(converted_number) => converted_number,
        Err(error) => panic!("Por favor, forneça um número. O argumento passado foi {}. Erro: {}", args[0], error)
    };

    if number % 2 == 0{
        println!("O número é divisivel por 2.");
    } else {
        println!("O número não é divisivel por 2.");
    }
}
