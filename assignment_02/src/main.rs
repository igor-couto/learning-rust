// Construir um algoritmo para ler 5 valores inteiros, calcular e imprimir a soma desses valores.

use::std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let n1 = args[1].parse::<i32>().unwrap();
    let n2 = args[2].parse::<i32>().unwrap();
    let n3 = args[3].parse::<i32>().unwrap();
    let n4 = args[4].parse::<i32>().unwrap();
    let n5 = args[5].parse::<i32>().unwrap();
    
    let sum = n1 + n2 + n3 + n4 + n5;
    
    println!("O resultado é {}", sum);
}
