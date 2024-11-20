//! Implementación del buffer compartido para el problema de productores y consumidores.

use std::sync::{Arc, Mutex};

/// Representa un buffer compartido entre productores y consumidores.
#[derive(Clone)]
pub struct Buffer {
    buffer: Arc<Mutex<Vec<String>>>,
}

impl Buffer {
    /// Crea un nuevo buffer con capacidad limitada.
    ///
    /// # Parámetros
    /// - `capacity`: Tamaño máximo del buffer.
    ///
    /// # Retorno
    /// Retorna una nueva instancia del buffer.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
        }
    }

    /// Añade un mensaje al buffer.
    ///
    /// # Parámetros
    /// - `message`: Mensaje a añadir.
    pub fn add(&self, message: String) {
        let mut buf = self.buffer.lock().unwrap();
        buf.push(message);
    }

    /// Elimina y retorna un mensaje del buffer.
    ///
    /// # Retorno
    /// El mensaje removido.
    pub fn remove(&self) -> Option<String> {
        let mut buf = self.buffer.lock().unwrap();
        if buf.is_empty() {
            None
        } else {
            Some(buf.remove(0))
        }
    }
}
