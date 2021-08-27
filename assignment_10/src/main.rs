/* 
Fazer um algoritmo para ler dois números e um dos símbolos das operações: +, -, * e /. 
Imprimir o resultado da operação efetuada sobre os números lidos.
*/

use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    let first_number = read_number(&stdin).expect("Número inválido");
    let operator = read_operator(&stdin).expect("Operador inválido");
    let second_number = read_number(&stdin).expect("Número inválido");

    let result = match operator.as_str() {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => panic!("O operador escolhido é inválido")
    };

    print!("O resultado é {}", result);
}

fn read_number(stdin: &io::Stdin) -> io::Result<i32> {
    print!("Digite um número: ");
    io::stdout().flush()?;
    
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim_end().parse::<i32>().unwrap())
}

fn read_operator(stdin: &io::Stdin) -> io::Result<String> {
    print!("Digite uma operação: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}