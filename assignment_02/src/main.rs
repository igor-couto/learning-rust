// Construir um algoritmo para ler 5 valores inteiros, calcular e imprimir a soma desses valores.

fn main() {
    
    let n1 = 1;
    let n2 = 2;
    let n3 = 3;
    let n4 = 4;
    let n5 = 5;
    
    let sum = n1 + n2 + n3 + n4 + n5;
    
    println!("O resultado é {}", vectorApproach());
}

fn vectorApproach() -> i32
{
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for number in numbers
    {
        sum += number;
    } 

    sum
}
