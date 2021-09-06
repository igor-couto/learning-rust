extern crate rand;

use std::fs::{self, File};
use std::io::Read;

use rand::Rng;

pub fn pick_a_word() -> &'static str {
    // TODO - ler a partir do arquivo

    let words: Vec<&str> = "ant baboon badger bat bear beaver camel cat clam cobra cougar
        coyote crow deer dog donkey duck eagle ferret fox frog goat
        goose hawk lion lizard llama mole monkey moose mouse mule newt
        otter owl panda parrot pigeon python rabbit ram rat raven
        rhino salmon seal shark sheep skunk sloth snake spider
        stork swan tiger toad trout turkey turtle weasel whale wolf
        wombat zebra"
        .split_whitespace()
        .collect();

    words[rand::thread_rng().gen_range(0..words.len())]

    // let mut file = File::open("words.txt").expect(&format!(
    //     "Arquivo com as palavras não foi encontrado: words.txt"
    // ));

    // let mut file_content = String::new();

    // file.read_to_string(&mut file_content)
    //     .expect("Falha ao ler arquivo.");

    // let words = file_content.as_str(); //as_str().split_whitespace().collect();

    // let aaa: Vec<&str> = words.split_whitespace().collect();

    // let random = rand::thread_rng().gen_range(0..=words.len());

    // let aaaa = (words.char_indices())[2];
}
