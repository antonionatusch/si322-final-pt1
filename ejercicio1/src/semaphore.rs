//! Implementación de un wrapper para manejar semáforos.

use std::sync::Arc;
use tokio::sync::Semaphore;

/// Wrapper alrededor de los semáforos para facilitar su uso.
#[derive(Clone)]
pub struct CustomSemaphore {
    semaphore: Arc<Semaphore>,
}

impl CustomSemaphore {
    /// Crea un nuevo semáforo con un número inicial de permisos.
    ///
    /// # Parámetros
    /// - `permits`: Número inicial de permisos del semáforo.
    ///
    /// # Retorno
    /// Retorna una nueva instancia de `CustomSemaphore`.
    pub fn new(permits: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(permits)),
        }
    }

    /// Adquiere un permiso del semáforo. Bloquea hasta que un permiso esté disponible.
    pub async fn acquire(&self) {
        self.semaphore.acquire().await.unwrap().forget();
    }

    /// Libera un permiso, incrementando la cantidad de permisos disponibles.
    ///
    /// # Parámetros
    /// - `permits`: Número de permisos a liberar.
    pub fn release(&self, permits: usize) {
        self.semaphore.add_permits(permits);
    }
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}
