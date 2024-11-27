use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::semaforo_operarios::SemaforOperarios;

pub async fn op1(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        // Producir una rueda
        println!("OP1 (Iteración {}/{}): Produciendo una rueda...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(2)).await;

        // Notificar al montador que una rueda está lista
        sync.rueda_producida().await;
        println!("OP1 (Iteración {}/{}): Rueda lista.", iteracion, max_iteraciones);
    }
    println!("OP1: Completó sus {} iteraciones y se detiene.", max_iteraciones);
}

pub async fn op2(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        // Producir un cuadro
        println!("OP2 (Iteración {}/{}): Produciendo un cuadro...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(3)).await;

        // Notificar al montador que un cuadro está listo
        sync.cuadro_producido().await;
        println!("OP2 (Iteración {}/{}): Cuadro listo.", iteracion, max_iteraciones);
    }
    println!("OP2: Completó sus {} iteraciones y se detiene.", max_iteraciones);
}

pub async fn op3(sync: Arc<SemaforOperarios>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        // Producir un manillar
        println!("OP3 (Iteración {}/{}): Produciendo un manillar...", iteracion, max_iteraciones);
        sleep(Duration::from_secs(1)).await;

        // Notificar al montador que un manillar está listo
        sync.manillar_producido().await;
        println!("OP3 (Iteración {}/{}): Manillar lista.", iteracion, max_iteraciones);
    }
    println!("OP3: Completó sus {} iteraciones y se detiene.", max_iteraciones);
}
