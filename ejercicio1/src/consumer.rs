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

    /// Consume mensajes utilizando el planteamiento original.
    pub async fn consume(&self) {
        loop {
            self.full_slots.acquire().await; // Verificar que hay un mensaje disponible
            if let Some(message) = self.buffer.remove().await {
                println!("Mensaje consumido: {}", message);
            }
            self.empty_slots.release(1); // Notificar que hay un espacio disponible
            sleep(Duration::from_secs(1)).await; // Simular tiempo de consumo
        }
    }

    /// Consume mensajes utilizando el planteamiento corregido.
    ///
    /// # Nota
    /// Este método corrige el orden de los semáforos para evitar condiciones de carrera.
    pub async fn consume_corregido(&self) {
        loop {
            self.full_slots.acquire().await; // Verificar que hay un mensaje disponible
            if let Some(message) = self.buffer.remove().await {
                println!("Mensaje consumido: {}", message);
            }
            self.empty_slots.release(1); // Notificar que hay un espacio disponible
            sleep(Duration::from_secs(1)).await; // Simular tiempo de consumo
        }
    }
}
