//Fazer um algoritmo para gerar e imprimir o resultado do número H, sendo H = 1 + 1/2 +1/3 + 1/4 + 1/5.

fn main() {
    let mut h = 0.0;

    for divider in 1..6 {
        println!("divider {}", divider);
        h += 1.0 / divider as f32;
    }

    println!("O número h é {}", h);

    assert!(h == 1.0 + 1.0/2.0 + 1.0/3.0 + 1.0/4.0 + 1.0/5.0);
}
