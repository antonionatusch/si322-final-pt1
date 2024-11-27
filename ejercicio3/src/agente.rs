use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::semaforo_fumadores::SemphoreSmoker;

pub async fn agente(sync: Arc<SemphoreSmoker>, max_iteraciones: usize, tiempo_espera: Duration) {
    let ingredientes = ["Papel y Tabaco", "Papel y Fósforos", "Tabaco y Fósforos"];

    for iteracion in 1..=max_iteraciones {
        println!("Iteración {}/{}: Agente comenzando.", iteracion, max_iteraciones);

        for ingredientes_colocados in &ingredientes {
            println!("Agente: Colocando {}", ingredientes_colocados);

            // Elige qué ingredientes colocar
            match *ingredientes_colocados {
                "Papel y Tabaco" => sync.colocar_papel_tabaco().await,
                "Papel y Fósforos" => sync.colocar_papel_fosforos().await,
                "Tabaco y Fósforos" => sync.colocar_tabaco_fosforos().await,
                _ => unreachable!(),
            }

            // Espera a que un fumador termine de fumar
            sync.esperar_fumador().await;
            println!("Agente: Un fumador terminó de fumar, repitiendo ciclo.");
        }

        // Espera antes de iniciar la siguiente iteración
        println!(
            "Agente: Finalizó la iteración {}/{}. Esperando {} segundos antes de continuar...",
            iteracion,
            max_iteraciones,
            tiempo_espera.as_secs()
        );
        sleep(tiempo_espera).await;
    }

    println!("Agente: Completó las {} iteraciones. Terminando proceso.", max_iteraciones);
}
