use std::sync::Arc; // Arc para compartir datos entre tareas asincrónicas.
use tokio::time::{sleep, Duration}; // Herramientas de Tokio para manejar tiempos asincrónicos.
use crate::semaforo_operarios::SemaforOperarios; // Importa el semáforo personalizado para coordinar a los operarios y al montador.

/// Función asincrónica que representa la lógica del montador en el problema de ensamblaje.
///
/// El montador espera a que los operarios produzcan las piezas necesarias (ruedas, cuadro y manillar)
/// para luego ensamblar una bicicleta. Este proceso se repite durante un número específico de iteraciones.
///
/// # Parámetros
/// - `sync`: Referencia compartida al semáforo que coordina las acciones de los operarios y el montador.
/// - `max_iteraciones`: Número máximo de bicicletas que ensamblará el montador.
///
/// # Comportamiento
/// - Espera a que las piezas necesarias estén disponibles.
/// - Ensambla la bicicleta.
/// - Repite el proceso hasta completar las iteraciones especificadas.
pub async fn montador(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    // Bucle que controla el número de iteraciones del montador.
    for iteracion in 1..=max_iteraciones {
        println!("Montador (Iteración {}/{}): Esperando piezas...", iteracion, max_iteraciones);

        // Espera las piezas necesarias: dos ruedas, un cuadro y un manillar.
        sync.esperar_piezas().await;

        // Simula el proceso de ensamblaje de la bicicleta.
        println!("Montador (Iteración {}/{}): Armando bicicleta...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(4)).await; // Simula el tiempo necesario para ensamblar la bicicleta.

        // Indica que la bicicleta está lista.
        println!("Montador (Iteración {}/{}): Bicicleta lista.", iteracion, max_iteraciones);
    }

    // Mensaje final indicando que el montador completó todas las iteraciones.
    println!("Montador: Completó las {} iteraciones. Terminando trabajo.", max_iteraciones);
}
