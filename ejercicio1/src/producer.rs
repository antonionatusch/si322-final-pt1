//! Implementación del productor.

use crate::buffer::Buffer;
use crate::semaphore::CustomSemaphore;
use tokio::time::{sleep, Duration};

/// Representa un productor que genera mensajes.
pub struct Producer {
    buffer: Buffer,
    empty_slots: CustomSemaphore,
    full_slots: CustomSemaphore,
}

impl Producer {
    /// Crea una nueva instancia del productor.
    ///
    /// # Parámetros
    /// - `buffer`: Buffer compartido.
    /// - `empty_slots`: Semáforo para espacios vacíos.
    /// - `full_slots`: Semáforo para espacios llenos.
    pub fn new(buffer: Buffer, empty_slots: CustomSemaphore, full_slots: CustomSemaphore) -> Self {
        Self {
            buffer,
            empty_slots,
            full_slots,
        }
    }

    /// Comienza a producir mensajes.
    pub async fn produce(&self) {
        loop {
            // Esperar espacio vacío
            self.empty_slots.acquire().await;

            // Producir un mensaje
            let message = "Mensaje producido".to_string();
            println!("Produciendo: {}", message);

            // Añadir al buffer
            self.buffer.add(message);

            // Liberar un espacio lleno
            self.full_slots.release(1);

            // Simular tiempo de producción
            sleep(Duration::from_secs(1)).await;
        }
    }
}
