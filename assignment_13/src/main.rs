/*
Elaborar um algoritmo para ler o código de um produto e informar a sua origem:
a) Código do produto entre 1 e 20: Europa
b) Código do produto entre 21 e 40: Ásia
c) Código do produto entre 41 e 60: América
d) Código do produto entre 61 e 80: África
e) Código do produto maior que 80: Paraguai
*/

use std::io::{self, Write};

// TODO : Verificar se isso funciona. Parece que não, mas seria bom.
// struct CountryRanges {
//     Europa = (0u8..20u8),
//     Asia = (21u8..40u8),
//     America = (41u8..60u8),
//     Africa = (61u8..80u8),
//     Paraguai = (8u8..),
// }

fn main() {
    let product_code = read_product_code().unwrap();

    println!("Local: {}", get_location(&product_code));

    // TODO : Verificar se isso funciona. Parece que não, mas seria bom.
    // match product_code {
    //     europa => println!("Europa"),
    //     asia => println!("Ásia"),
    //     america => println!("América"),
    //     africa => println!("Africa"),
    //     paraguai => println!("Paraguai"),
    //     _ => println!("Código do produto não está relacionado a nenhum lugar"),
    // };
}

fn read_product_code() -> io::Result<u8> {
    print!("Digite o código do produto: ");
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut buffer: String = String::new();

    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim_end().parse::<u8>().unwrap())
}

fn get_location(code: &u8) -> String {
    let europa = 0u8..=20u8;
    let asia = 21u8..=40u8;
    let america = 41u8..=60u8;
    let africa = 61u8..=80u8;
    let paraguai = 8u8..;

    if europa.contains(code) {
        return String::from("Europa");
    }
    if asia.contains(code) {
        return String::from("Asia");
    }
    if america.contains(code) {
        return String::from("América");
    }
    if africa.contains(code) {
        return String::from("Africa");
    }
    if paraguai.contains(code) {
        return String::from("Paraguai");
    }

    String::from("Código do produto não está relacionado a nenhum lugar")
}
