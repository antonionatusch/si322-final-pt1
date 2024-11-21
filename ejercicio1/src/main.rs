mod buffer;
mod consumer;
mod menu;
mod producer;
mod semaphore;

use menu::{
    problema_1_planteamiento_correcto, problema_1_planteamiento_inicial,
    problema_2_sincronizacion_de_secuencias,
};
use std::io;

/// Punto de entrada del programa.
#[tokio::main]
async fn main() {
    loop {
        // Mostrar el menú
        println!("Seleccione una opción:");
        println!("1. Problema 1: Planteamiento Inicial");
        println!("2. Problema 1: Planteamiento Correcto");
        println!("3. Problema 2: Sincronización de Secuencias (Ejercicio 8)");
        println!("0. Salir");

        // Leer la opción del usuario
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error al leer la entrada");

        match option.trim() {
            "1" => {
                // Ejecutar la opción "Problema 1: Planteamiento Inicial"
                problema_1_planteamiento_inicial().await;
            }
            "2" => {
                // Ejecutar la opción "Problema 1: Planteamiento Correcto"
                problema_1_planteamiento_correcto().await;
            }
            "3" => {
                // Ejecutar la opción "Problema 2: Sincronización de Secuencias (Ejercicio 8)"
                problema_2_sincronizacion_de_secuencias().await;
            }
            "0" => {
                // Salir del programa
                println!("Saliendo...");
                break;
            }
            _ => {
                println!("Opción no válida. Por favor, intente de nuevo.");
            }
        }
    }
}
