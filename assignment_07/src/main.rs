//Ler um número inteiro e informar se ele é divisível por 2 e por 3 simultaneamente.

use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    check_number_of_parameters(args.len());

    let number = convert_argument_to_number(&args[0]);

    if number % 2 == 0 && number % 3 == 0 {
        println!("O número é divisivel por 2 e por 3 simultaneamente");
    } else {
        println!("O número não é divisivel por 2 e por 3 simultaneamente");
    }
}

fn check_number_of_parameters( number_of_args : usize) {
    if number_of_args !=  1 {
        panic!("Por favor, informe 1 e apenas 1 argumento. Foram encontrados {} argumentos.", number_of_args);
    }
}

fn convert_argument_to_number(argument: &String) -> i32 {
    return match argument.parse::<i32>() {
        Ok(converted_number) => converted_number,
        Err(error) => panic!("Por favor, forneça um número. O argumento passado foi {}. Erro: {}", argument, error)
    };
}