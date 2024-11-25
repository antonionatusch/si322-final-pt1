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
            println!("[Original] Intentando adquirir mensaje disponible...");
            self.full_slots.acquire().await; // Verificar que hay un mensaje disponible
            println!("[Original] Mensaje disponible adquirido. Consumiendo mensaje...");
            if let Some(message) = self.buffer.remove().await {
                println!("[Original] Mensaje consumido: {}", message);
            } else {
                println!("[Original] No se encontró mensaje en el buffer.");
            }
            self.empty_slots.release(1); // Notificar que hay un espacio disponible
            println!("[Original] Notificación enviada para espacio vacío.");
            tokio::time::sleep(Duration::from_secs(1)).await; // Simular tiempo de consumo
        }
    }

    /// Consume mensajes utilizando el planteamiento corregido.
    ///
    /// # Nota
    /// Este método corrige el manejo de mensajes consumidos y valida el contenido del mensaje.
    pub async fn consume_corregido(&self) {
        loop {
            println!("[Corregido] Intentando adquirir mensaje disponible...");
            self.full_slots.acquire().await; // Verificar que hay un mensaje disponible

            if let Some(message) = self.buffer.remove().await {
                if message.contains("Error") {
                    println!("[Corregido] Mensaje inválido encontrado: {}", message);
                    continue; // Ignorar mensajes inválidos
                }
                println!("[Corregido] Mensaje consumido: {}", message);
            } else {
                println!("[Corregido] No se encontró mensaje en el buffer.");
            }

            self.empty_slots.release(1); // Notificar que hay un espacio disponible
            sleep(Duration::from_secs(1)).await; // Simular tiempo de consumo
        }
    }
}
