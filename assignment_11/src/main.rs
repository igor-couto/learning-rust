/*
Os funcionários de uma empresa receberam um aumento
de salário: técnicos (código = 1), 50%; gerentes (código =
2), 30%; demais funcionários (código = 3), 20%. Escrever
um algoritmo para ler o código do cargo de um
funcionário e o valor do seu salário atual, calcular e
imprimir o novo salário após o aumento) 
*/

use std::io::{self, Write};

fn main() {
    
    let stdin = io::stdin();

    let position_code = get_position_code(&stdin).expect("Falha ao obter o código do cargo do funcionario.");
    let wage = get_wage(&stdin).expect("Falha ao obter salário do funcionário.");
    
    let new_salary = match position_code {
        1 => wage * 1.5,
        2 => wage * 1.2,
        3 => wage * 1.3,
        _ => panic!("Código do funcionário não experado.")
    };

    print!("O novo salário é {}", new_salary)
}

fn get_position_code(stdin: &io::Stdin) -> io::Result<u8>{
    print!("Código do cargo: ");
    io::stdout().flush()?;
    
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<u8>().unwrap())
}

fn get_wage(stdin: &io::Stdin) -> io::Result<f32> {
    print!("Salário: ");
    io::stdout().flush()?;
    
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<f32>().unwrap())
}