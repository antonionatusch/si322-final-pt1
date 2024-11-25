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

    /// Produce mensajes utilizando el planteamiento original.
    pub async fn produce(&self) {
        loop {
            println!("[Original] Intentando adquirir espacio vacío...");
            self.empty_slots.acquire().await; // Verificar espacio disponible
            println!("[Original] Espacio vacío adquirido. Produciendo mensaje...");
            self.buffer.add("Mensaje producido".to_string()).await;
            println!("[Original] Mensaje producido y añadido al buffer.");
            self.full_slots.release(1); // Notificar que hay un mensaje disponible
            println!("[Original] Notificación enviada para espacio lleno.");
            tokio::time::sleep(Duration::from_secs(1)).await; // Simular tiempo de producción
        }
    }

    /// Produce mensajes utilizando el planteamiento corregido.
    ///
    /// # Nota
    /// Este método corrige el orden de los semáforos para evitar condiciones de carrera
    /// y agrega validación de espacios disponibles.
    pub async fn produce_corregido(&self) {
        loop {
            println!("[Corregido] Intentando adquirir espacio vacío...");
            let available = self.empty_slots.available_permits(); // Validar espacios disponibles
            if available == 0 {
                println!("[Corregido] Buffer lleno, esperando...");
                sleep(Duration::from_millis(500)).await;
                continue;
            }

            self.empty_slots.acquire().await; // Verificar espacio disponible
            self.buffer
                .add("Mensaje producido (corregido)".to_string())
                .await; // Añadir al buffer
            self.full_slots.release(1); // Notificar que hay un mensaje disponible
            println!("[Corregido] Producción completada.");
            sleep(Duration::from_secs(1)).await; // Simular tiempo de producción
        }
    }
}
