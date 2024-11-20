//! Implementación del consumidor.

use crate::buffer::Buffer;
use crate::semaphore::CustomSemaphore;
use tokio::time::{sleep, Duration};

/// Representa un consumidor que procesa mensajes.
pub struct Consumer {
    buffer: Buffer,
    empty_slots: CustomSemaphore,
    full_slots: CustomSemaphore,
}

impl Consumer {
    /// Crea una nueva instancia del consumidor.
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

    /// Comienza a consumir mensajes.
    pub async fn consume(&self) {
        loop {
            // Esperar espacio lleno
            self.full_slots.acquire().await;

            // Consumir un mensaje
            if let Some(message) = self.buffer.remove() {
                println!("Consumiendo: {}", message);
            }

            // Liberar un espacio vacío
            self.empty_slots.release(1);

            // Simular tiempo de consumo
            sleep(Duration::from_secs(1)).await;
        }
    }
}
