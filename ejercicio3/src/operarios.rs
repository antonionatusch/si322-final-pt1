use std::sync::Arc; // Arc para compartir datos entre tareas asincrónicas.
use tokio::time::{sleep, Duration}; // Herramientas de Tokio para manejar tiempos asincrónicos.
use crate::semaforo_operarios::SemaforOperarios; // Importa el semáforo personalizado para coordinar a los operarios y el montador.

/// Función asincrónica que representa la lógica del Operario 1 (OP1).
///
/// OP1 produce ruedas y notifica al montador cuando cada rueda está lista.
///
/// # Parámetros
/// - `sync`: Referencia compartida al semáforo que coordina las acciones entre operarios y montador.
/// - `max_iteraciones`: Número máximo de ruedas que producirá OP1.
///
/// # Comportamiento
/// - Produce una rueda.
/// - Notifica al montador que una rueda está lista.
/// - Repite el proceso hasta completar las iteraciones especificadas.
pub async fn op1(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        // Simula la producción de una rueda.
        println!("OP1 (Iteración {}/{}): Produciendo una rueda...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(2)).await;

        // Notifica al montador que una rueda está lista.
        sync.rueda_producida().await;
        println!("OP1 (Iteración {}/{}): Rueda lista.", iteracion, max_iteraciones);
    }

    // Mensaje final indicando que OP1 completó todas las iteraciones.
    println!("OP1: Completó sus {} iteraciones y se detiene.", max_iteraciones);
}

/// Función asincrónica que representa la lógica del Operario 2 (OP2).
///
/// OP2 produce cuadros y notifica al montador cuando cada cuadro está listo.
///
/// # Parámetros
/// - `sync`: Referencia compartida al semáforo que coordina las acciones entre operarios y montador.
/// - `max_iteraciones`: Número máximo de cuadros que producirá OP2.
///
/// # Comportamiento
/// - Produce un cuadro.
/// - Notifica al montador que un cuadro está listo.
/// - Repite el proceso hasta completar las iteraciones especificadas.
pub async fn op2(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        // Simula la producción de un cuadro.
        println!("OP2 (Iteración {}/{}): Produciendo un cuadro...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(3)).await;

        // Notifica al montador que un cuadro está listo.
        sync.cuadro_producido().await;
        println!("OP2 (Iteración {}/{}): Cuadro listo.", iteracion, max_iteraciones);
    }

    // Mensaje final indicando que OP2 completó todas las iteraciones.
    println!("OP2: Completó sus {} iteraciones y se detiene.", max_iteraciones);
}

/// Función asincrónica que representa la lógica del Operario 3 (OP3).
///
/// OP3 produce manillares y notifica al montador cuando cada manillar está listo.
///
/// # Parámetros
/// - `sync`: Referencia compartida al semáforo que coordina las acciones entre operarios y montador.
/// - `max_iteraciones`: Número máximo de manillares que producirá OP3.
///
/// # Comportamiento
/// - Produce un manillar.
/// - Notifica al montador que un manillar está listo.
/// - Repite el proceso hasta completar las iteraciones especificadas.
pub async fn op3(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        // Simula la producción de un manillar.
        println!("OP3 (Iteración {}/{}): Produciendo un manillar...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(1)).await;

        // Notifica al montador que un manillar está listo.
        sync.manillar_producido().await;
        println!("OP3 (Iteración {}/{}): Manillar lista.", iteracion, max_iteraciones);
    }

    // Mensaje final indicando que OP3 completó todas las iteraciones.
    println!("OP3: Completó sus {} iteraciones y se detiene.", max_iteraciones);
}
