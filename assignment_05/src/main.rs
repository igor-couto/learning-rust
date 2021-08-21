//Ler dois números inteiros e informar se o primeiro valor lido é maior, menor ou igual ao segundo.

use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() != 2 {
        println!("Forneça 2 parâmetros para o programa. Foram passados {} parâmetros.", args.len());
        return;
    }

    let first_number = args[0].parse::<i32>().unwrap();
    let second_number = args[1].parse::<i32>().unwrap();

    if first_number < second_number {
        println!("O primeiro número é menor do que o segundo");
        return;
    }
    else if first_number > second_number {
        println!("O primeiro número é maior do que o segundo");
        return;
    }
    println!("O primeiro número é igual ao segundo");
}
