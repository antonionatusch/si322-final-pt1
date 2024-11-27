use tokio::sync::Semaphore; // Semaphore de Tokio para gestionar la concurrencia asincrónica.
use std::sync::Arc; // Arc para compartir datos entre tareas asincrónicas.

/// Estructura que representa un semáforo para controlar el acceso a recursos compartidos.
///
/// El semáforo limita el número de tareas que pueden acceder simultáneamente
/// a una sección crítica o recurso. Es útil para evitar interbloqueos y manejar la concurrencia.
pub struct Semaforo {
    semaforo: Arc<Semaphore>, // Semáforo compartido utilizando un Arc.
}

impl Semaforo {
    /// Constructor que inicializa un nuevo semáforo.
    ///
    /// # Parámetros
    /// - `max_permits`: Número máximo de permisos disponibles para el semáforo.
    ///
    /// # Retorno
    /// Retorna una instancia de `Semaforo` con el número especificado de permisos.
    pub fn new(max_permits: usize) -> Self {
        Self {
            semaforo: Arc::new(Semaphore::new(max_permits)), // Inicializa el semáforo con permisos máximos.
        }
    }

    /// Método asincrónico para adquirir un permiso del semáforo.
    ///
    /// Si no hay permisos disponibles, la tarea esperará hasta que un permiso sea liberado.
    pub async fn acquire(&self) {
        self.semaforo.acquire().await.unwrap(); // Espera hasta adquirir un permiso.
    }

    /// Método para liberar un permiso al semáforo.
    ///
    /// Incrementa el número de permisos disponibles en el semáforo.
    pub fn release(&self) {
        self.semaforo.add_permits(1); // Agrega un permiso al semáforo.
    }
}
