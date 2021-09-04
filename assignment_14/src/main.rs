/*
Para auxiliar os vendedores de uma loja na orientação
aos clientes sobre as diversas formas de pagamento,
desenvolver um algoritmo para imprimir o seguinte menu:
Forma de pagamento:
1. A vista.
2. Cheque para trinta dias.
3. Em duas vezes.
4. Em três vezes.
5. Em quatro vezes.
6. A partir de cinco vezes.
Entre com sua opção:

E em seguida ler o código da opção de pagamento e imprimir
uma das mensagens de acordo com a opção lida.
*/

use std::io;

fn main() {
    const PAYMENT_METHOD: [&str; 6] = [
        "A vista.",
        "Cheque para trinta dias.",
        "Em duas vezes.",
        "Em três vezes.",
        "Em quatro vezes.",
        "A partir de cinco vezes.",
    ];

    show_options(&PAYMENT_METHOD);

    let selected_option = get_input().unwrap();

    show_result(selected_option, &PAYMENT_METHOD);
}

fn show_options(payments: &[&str; 6]) {
    println!(
        "Forma de pagamento:
        1. {}
        2. {}
        3. {}
        4. {}
        5. {}
        6. {}",
        payments[0], payments[1], payments[2], payments[3], payments[4], payments[5]
    );
}

fn get_input() -> io::Result<u8> {
    println!("Entre com sua opção: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<u8>().unwrap())
}

fn show_result(selected_option: u8, payments: &[&str; 6]) {
    println!("{}", payments[(selected_option - 1) as usize]);
}
