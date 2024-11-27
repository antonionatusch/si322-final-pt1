use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::semaforo_operarios::SemaforOperarios;

pub async fn montador(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        println!("Montador (Iteración {}/{}): Esperando piezas...", iteracion, max_iteraciones);

        // Esperar las dos ruedas, un cuadro, y un manillar
        sync.esperar_piezas().await;

        println!("Montador (Iteración {}/{}): Armando bicicleta...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(4)).await;

        println!("Montador (Iteración {}/{}): Bicicleta lista.", iteracion, max_iteraciones);
    }

    println!("Montador: Completó las {} iteraciones. Terminando trabajo.", max_iteraciones);
}
