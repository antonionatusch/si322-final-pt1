/// Módulos que encapsulan la lógica de la mesa, los filósofos y su sincronización.
mod mesa; // Módulo que define la mesa y sus operaciones.
mod filosofos; // Módulo que define la lógica de los filósofos.
mod estados; // Módulo que define los estados posibles de los filósofos.
mod semaforo_filosofos; // Módulo para la sincronización de los filósofos mediante semáforos.

use crate::mesa::Mesa; // Importa la estructura Mesa desde el módulo correspondiente.
use crate::filosofos::filosofo; // Importa la función que maneja la lógica de los filósofos.
use std::sync::Arc; // Para compartir datos entre hilos de manera segura.
use tokio; // Tokio es utilizado para manejar la concurrencia asincrónica.

#[tokio::main]
/// Función principal que inicializa y ejecuta el problema de los filósofos comensales.
///
/// Esta función configura la mesa, los filósofos, y maneja su sincronización utilizando Tokio.
///
/// # Configuración
/// - Número de filósofos: `N = 5`.
/// - Máximo de filósofos comiendo simultáneamente: `max_comiendo = 2`.
async fn main() {
    const N: usize = 5; // Número de filósofos y tenedores.
    let max_comiendo = 2; // Máximo número de filósofos que pueden comer simultáneamente.

    // Inicializa la mesa compartida con la restricción de `max_comiendo`.
    let mesa = Arc::new(Mesa::new(max_comiendo));

    // Vector para almacenar los manejadores de las tareas asincrónicas.
    let mut handles = vec![];

    // Crea y lanza tareas asincrónicas para cada filósofo.
    for i in 0..N {
        let mesa_clone = Arc::clone(&mesa); // Clona la referencia compartida de la mesa.
        handles.push(tokio::spawn(async move {
            filosofo(i, mesa_clone).await; // Ejecuta la lógica del filósofo `i`.
        }));
    }

    // Espera a que todas las tareas (filósofos) terminen su ejecución.
    for handle in handles {
        handle.await.unwrap(); // Verifica que las tareas terminen correctamente.
    }
}
