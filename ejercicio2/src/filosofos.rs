use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::mesa::Mesa;

pub async fn filosofo(id: usize, mesa: Arc<Mesa>) {
    const MAX_ITERACIONES: usize = 5; // Máximo número de iteraciones por filósofo

    for iteracion in 1..=MAX_ITERACIONES {
        println!("Iteración {}: Filósofo {} está pensando.", iteracion, id);
        sleep(Duration::from_secs(2)).await; // Tiempo para pensar

        println!("Iteración {}: Filósofo {} tiene hambre.", iteracion, id);
        mesa.tomar_tenedores(id).await;

        println!("Iteración {}: Filósofo {} está comiendo.", iteracion, id);
        sleep(Duration::from_secs(3)).await; // Tiempo para comer

        mesa.dejar_tenedores(id);
        println!("Iteración {}: Filósofo {} ha terminado de comer.", iteracion, id);
    }

    println!("Filósofo {} ha completado sus 5 iteraciones y se retira.", id);
}
