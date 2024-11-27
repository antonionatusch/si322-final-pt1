mod barber;
mod barber_impl;
mod read_write_lock;
mod read_write_lock_impl;
mod menu;

use std::io;

#[tokio::main]
async fn main() {
    let mut answer = String::new();

    loop {
        println!("===== Problemas con semáforos en Rust =====");
        println!("Elija una opción:");
        println!("1. Problema 12");
        println!("2. Problema 13");
        println!("0. Salir");

        answer.clear();
        io::stdin()
            .read_line(&mut answer)
            .expect("Fallo al leer la entrada");

        match answer.trim() {
            "1" => menu::problem_12().await,
            "2" => menu::problem_13().await,
            "0" => {
                println!("Hasta luego.");
                break;
            }
            _ => println!("Opción no válida, intente de nuevo."),
        }
    }
}
