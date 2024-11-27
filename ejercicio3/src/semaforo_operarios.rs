use tokio::sync::Semaphore; // Semaphore de Tokio para manejar la concurrencia asincrónica.
use std::sync::Arc; // Arc para compartir datos entre tareas asincrónicas.

/// Estructura que representa el sistema de sincronización para operarios y el montador.
///
/// Este sistema utiliza semáforos para coordinar la producción de piezas por parte
/// de los operarios (ruedas, cuadros y manillares) y su ensamblaje por el montador.
pub struct SemaforOperarios {
    ruedas: Arc<Semaphore>,   // Semáforo para controlar la disponibilidad de ruedas.
    cuadros: Arc<Semaphore>,  // Semáforo para controlar la disponibilidad de cuadros.
    manillares: Arc<Semaphore>, // Semáforo para controlar la disponibilidad de manillares.
}

impl SemaforOperarios {
    /// Constructor que inicializa un nuevo sistema de sincronización.
    ///
    /// # Retorno
    /// Retorna una instancia de `SemaforOperarios` con todos los semáforos inicializados a 0 permisos.
    pub fn new() -> Self {
        Self {
            ruedas: Arc::new(Semaphore::new(0)),     // Inicialmente, no hay ruedas disponibles.
            cuadros: Arc::new(Semaphore::new(0)),    // Inicialmente, no hay cuadros disponibles.
            manillares: Arc::new(Semaphore::new(0)), // Inicialmente, no hay manillares disponibles.
        }
    }

    /// Método para notificar que una rueda ha sido producida.
    ///
    /// Incrementa el semáforo correspondiente para indicar que una rueda está lista.
    pub async fn rueda_producida(&self) {
        self.ruedas.add_permits(1); // Añade un permiso para ruedas.
    }

    /// Método para notificar que un cuadro ha sido producido.
    ///
    /// Incrementa el semáforo correspondiente para indicar que un cuadro está listo.
    pub async fn cuadro_producido(&self) {
        self.cuadros.add_permits(1); // Añade un permiso para cuadros.
    }

    /// Método para notificar que un manillar ha sido producido.
    ///
    /// Incrementa el semáforo correspondiente para indicar que un manillar está listo.
    pub async fn manillar_producido(&self) {
        self.manillares.add_permits(1); // Añade un permiso para manillares.
    }

    /// Método para que el montador espere hasta que todas las piezas necesarias estén disponibles.
    ///
    /// El montador se bloquea hasta que:
    /// - Hay al menos 2 ruedas disponibles.
    /// - Hay al menos 1 cuadro disponible.
    /// - Hay al menos 1 manillar disponible.
    pub async fn esperar_piezas(&self) {
        // Adquiere permisos para dos ruedas.
        self.ruedas.acquire().await.unwrap();
        self.ruedas.acquire().await.unwrap();

        // Adquiere un permiso para un cuadro.
        self.cuadros.acquire().await.unwrap();

        // Adquiere un permiso para un manillar.
        self.manillares.acquire().await.unwrap();
    }
}
