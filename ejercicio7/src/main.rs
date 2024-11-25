mod menu;
mod server;
mod printer;
mod user;

use std::io;

#[tokio::main]
async fn main() {
    let mut answer = String::new();

    loop {
        println!("===== Problemas con semáforos en Rust =====");
        println!("Elija una opción:");
        println!("1. Problema 17");
        println!("2. Problema 18 A");
        println!("3. Problema 18 B");
        println!("0. Salir");

        answer.clear();
        io::stdin()
            .read_line(&mut answer)
            .expect("Fallo al leer la entrada");

        match answer.trim() {
            "1" => menu::problema_17().await,
            "2" => menu::problema_18_a(),
            "3" => menu::problema_18_b(),
            "0" => {
                println!("Hasta luego.");
                break;
            }
            _ => println!("Opción no válida, intente de nuevo."),
        }
    }
}
