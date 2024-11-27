use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tokio::sync::Notify;
use crate::semaforo_fumadores::SemphoreSmoker;

pub async fn fumador(tipo: &str, sync: Arc<SemphoreSmoker>, notify: Arc<Notify>, max_iteraciones: usize) {
    for iteracion in 1..=max_iteraciones {
        tokio::select! {
            _ = notify.notified() => {
                println!("Fumador con {}: Detenido por notificación.", tipo);
                break;
            }
            _ = async {
                // Esperar ingredientes específicos según el tipo de fumador
                match tipo {
                    "Papel" => sync.esperar_tabaco_fosforos().await,
                    "Tabaco" => sync.esperar_papel_fosforos().await,
                    "Fósforos" => sync.esperar_papel_tabaco().await,
                    _ => unreachable!(),
                }

                // Simular armado y fumar
                println!("Fumador con {} (Iteración {}/{}): Armando y fumando cigarrillo.", tipo, iteracion, max_iteraciones);
                sleep(Duration::from_secs(2)).await;

                // Notificar al agente que terminó
                sync.notificar_agente().await;
                println!("Fumador con {} (Iteración {}/{}): Terminó de fumar.", tipo, iteracion, max_iteraciones);
            } => {}
        }
    }

    println!("Fumador con {}: Completó sus {} iteraciones y se retira.", tipo, max_iteraciones);
}
