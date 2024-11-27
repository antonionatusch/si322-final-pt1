use std::sync::Arc; // Arc se utiliza para compartir el semáforo entre múltiples tareas.
use tokio::time::{sleep, Duration}; // Herramientas de Tokio para manejar tiempos asincrónicos.
use crate::semaforo_fumadores::SemphoreSmoker; // Importa el semáforo personalizado para la sincronización.

/// Función asincrónica que representa el comportamiento del agente en el problema de los fumadores.
///
/// El agente selecciona y coloca ingredientes para que los fumadores puedan armar y fumar cigarrillos.
/// Este proceso se repite durante un número específico de iteraciones.
///
/// # Parámetros
/// - `sync`: Referencia compartida al semáforo que coordina las acciones entre el agente y los fumadores.
/// - `max_iteraciones`: Número máximo de iteraciones que realizará el agente.
/// - `tiempo_espera`: Duración de la pausa entre cada iteración.
///
/// # Comportamiento
/// - El agente selecciona y coloca combinaciones de ingredientes.
/// - Espera a que un fumador termine de fumar antes de continuar.
/// - Pausa brevemente entre iteraciones para simular el tiempo de espera.
pub async fn agente(sync: Arc<SemphoreSmoker>, max_iteraciones: usize, tiempo_espera: Duration) {
    // Lista de combinaciones de ingredientes que el agente puede colocar.
    let ingredientes = ["Papel y Tabaco", "Papel y Fósforos", "Tabaco y Fósforos"];

    // Bucle que controla el número de iteraciones del agente.
    for iteracion in 1..=max_iteraciones {
        println!("Iteración {}/{}: Agente comenzando.", iteracion, max_iteraciones);

        // El agente coloca cada combinación de ingredientes en orden.
        for ingredientes_colocados in &ingredientes {
            println!("Agente: Colocando {}", ingredientes_colocados);

            // Determina qué combinación de ingredientes colocar y lo notifica a los fumadores.
            match *ingredientes_colocados {
                "Papel y Tabaco" => sync.colocar_papel_tabaco().await, // Notifica a los fumadores con fósforos.
                "Papel y Fósforos" => sync.colocar_papel_fosforos().await, // Notifica a los fumadores con tabaco.
                "Tabaco y Fósforos" => sync.colocar_tabaco_fosforos().await, // Notifica a los fumadores con papel.
                _ => unreachable!(), // Garantiza que no haya combinaciones no válidas.
            }

            // Espera a que un fumador termine de fumar antes de colocar nuevos ingredientes.
            sync.esperar_fumador().await;
            println!("Agente: Un fumador terminó de fumar, repitiendo ciclo.");
        }

        // Espera antes de iniciar la siguiente iteración para simular tiempo entre rondas.
        println!(
            "Agente: Finalizó la iteración {}/{}. Esperando {} segundos antes de continuar...",
            iteracion,
            max_iteraciones,
            tiempo_espera.as_secs()
        );
        sleep(tiempo_espera).await;
    }

    // Mensaje final indicando que el agente completó todas las iteraciones.
    println!("Agente: Completó las {} iteraciones. Terminando proceso.", max_iteraciones);
}
