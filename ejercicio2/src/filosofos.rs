use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::mesa::Mesa;

/// Función asincrónica que representa la lógica de un filósofo.
///
/// Cada filósofo alterna entre pensar, tener hambre y comer.
/// Este comportamiento se repite un número fijo de iteraciones.
///
/// # Parámetros
/// - `id`: Identificador único del filósofo (número entero).
/// - `mesa`: Referencia compartida a la mesa donde los filósofos interactúan.
///
/// # Comportamiento
/// - Un filósofo piensa, tiene hambre, toma los tenedores, come, y luego libera los tenedores.
/// - Este proceso se repite hasta alcanzar el número máximo de iteraciones.
pub async fn filosofo(id: usize, mesa: Arc<Mesa>) {
    const MAX_ITERACIONES: usize = 5; // Máximo número de iteraciones por filósofo.

    for iteracion in 1..=MAX_ITERACIONES {
        // Filósofo está pensando.
        println!("Iteración {}: Filósofo {} está pensando.", iteracion, id);
        sleep(Duration::from_secs(2)).await; // Simula el tiempo de pensar.

        // Filósofo tiene hambre e intenta tomar los tenedores.
        println!("Iteración {}: Filósofo {} tiene hambre.", iteracion, id);
        mesa.tomar_tenedores(id).await;

        // Filósofo está comiendo.
        println!("Iteración {}: Filósofo {} está comiendo.", iteracion, id);
        sleep(Duration::from_secs(3)).await; // Simula el tiempo de comer.

        // Filósofo termina de comer y libera los tenedores.
        mesa.dejar_tenedores(id);
        println!("Iteración {}: Filósofo {} ha terminado de comer.", iteracion, id);
    }

    // Filósofo completa sus iteraciones y se retira.
    println!("Filósofo {} ha completado sus {} iteraciones y se retira.", id, MAX_ITERACIONES);
}
