mod buffer;
mod producer;
mod consumer;
mod menu;
mod monitor;

use std::io;

fn main() {
    let mut answer = String::new();

    loop {
        println!("===== Problemas con semáforos en Rust =====");
        println!("Elija una opción:");
        println!("1. Problema 14");
        println!("2. Problema 15");
        println!("0. Salir");

        answer.clear();
        io::stdin()
            .read_line(&mut answer)
            .expect("Fallo al leer la entrada");

        match answer.trim() {
            "1" => menu::problema_14(),
            "2" => menu::problema_15(),
            "0" => {
                println!("Hasta luego.");
                break;
            }
            _ => println!("Opción no válida, intente de nuevo."),
        }
    }
}