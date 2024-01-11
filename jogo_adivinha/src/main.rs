extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Jogo de adivinhar numeros");

    let _numero_gerado = rand::thread_rng().gen_range(1, 100);
    let mut _tentativas = 0;
    loop {
        println!("");
        println!("Digite um numero: ");

        let mut _palpite = String::new();

        io::stdin()
            .read_line(&mut _palpite)
            .expect("Falha ao ler entrada");

        let _palpite: u32 = match _palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite foi {}", _palpite);
        _tentativas+=1;
        println!("Tentativas: {}", _tentativas);

        match _palpite.cmp(&_numero_gerado) {
            Ordering::Less => println!("Numero abaixo"),
            Ordering::Greater => println!("Numero acima"),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }
    }
}
