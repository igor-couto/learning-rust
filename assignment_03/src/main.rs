//Construir um algoritmo para ler 6 valores inteiros, calcular e imprimir a média aritmética desses valores.

use::std::env;

fn main() {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() != 6 {
        println!("Erro: Forneça 6 argumentos para o programa. Foram passados {} parâmetros", args.len());
        return;
    }

    let mut sum = 0;
    for number in args {
        sum += number.parse::<i32>().unwrap();
    }

    println!("A média aritimética é: {}", sum as f32 / 6.0);
}
