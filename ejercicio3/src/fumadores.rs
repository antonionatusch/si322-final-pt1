use std::sync::Arc; // Arc para compartir el semáforo entre múltiples tareas asincrónicas.
use tokio::time::{sleep, Duration}; // Herramientas para manejar tiempos y pausas asincrónicas.
use tokio::sync::Notify; // Notify para manejar notificaciones entre tareas.
use crate::semaforo_fumadores::SemphoreSmoker; // Importa el semáforo personalizado para fumadores.

/// Función asincrónica que representa la lógica de un fumador en el problema de sincronización.
///
/// Cada fumador tiene un tipo específico de recurso (Papel, Tabaco o Fósforos) y espera
/// que el agente coloque los otros dos ingredientes necesarios para armar y fumar un cigarrillo.
///
/// # Parámetros
/// - `tipo`: El tipo de fumador ("Papel", "Tabaco" o "Fósforos").
/// - `sync`: Referencia compartida al semáforo que coordina las acciones del fumador y el agente.
/// - `notify`: Referencia compartida para notificar al fumador de una interrupción.
/// - `max_iteraciones`: Número máximo de iteraciones que realizará el fumador.
///
/// # Comportamiento
/// - El fumador espera a que el agente coloque los ingredientes que necesita.
/// - Una vez que tiene los ingredientes, arma y fuma un cigarrillo.
/// - Notifica al agente que terminó para que el proceso continúe.
/// - Puede ser interrumpido por una notificación externa.
pub async fn fumador(tipo: &str, sync: Arc<SemphoreSmoker>, notify: Arc<Notify>, max_iteraciones: usize) {
    // Bucle principal: controla el número de iteraciones.
    for iteracion in 1..=max_iteraciones {
        tokio::select! {
            // Caso 1: Interrupción mediante notificación.
            _ = notify.notified() => {
                println!("Fumador con {}: Detenido por notificación.", tipo);
                break; // Sale del bucle si recibe una notificación.
            }
            // Caso 2: Fumador espera, arma y fuma un cigarrillo.
            _ = async {
                // Espera los ingredientes específicos según su tipo.
                match tipo {
                    "Papel" => sync.esperar_tabaco_fosforos().await, // Fumador con papel espera tabaco y fósforos.
                    "Tabaco" => sync.esperar_papel_fosforos().await, // Fumador con tabaco espera papel y fósforos.
                    "Fósforos" => sync.esperar_papel_tabaco().await, // Fumador con fósforos espera papel y tabaco.
                    _ => unreachable!(), // Garantiza que el tipo siempre sea válido.
                }

                // Simula el armado y consumo del cigarrillo.
                println!("Fumador con {} (Iteración {}/{}): Armando y fumando cigarrillo.", tipo, iteracion, max_iteraciones);
                sleep(Duration::from_secs(2)).await; // Simula el tiempo necesario para fumar.

                // Notifica al agente que terminó.
                sync.notificar_agente().await;
                println!("Fumador con {} (Iteración {}/{}): Terminó de fumar.", tipo, iteracion, max_iteraciones);
            } => {}
        }
    }

    // Mensaje final indicando que el fumador completó todas sus iteraciones.
    println!("Fumador con {}: Completó sus {} iteraciones y se retira.", tipo, max_iteraciones);
}
