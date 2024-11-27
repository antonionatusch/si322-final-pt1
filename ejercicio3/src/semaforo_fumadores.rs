use tokio::sync::Semaphore; // Semaphore de Tokio para gestionar la concurrencia asincrónica.
use std::sync::Arc; // Arc para compartir datos entre tareas asincrónicas.

/// Estructura que representa el sistema de sincronización entre el agente y los fumadores.
///
/// Este sistema utiliza semáforos para coordinar las acciones del agente
/// (que coloca los ingredientes) y los fumadores (que esperan ingredientes específicos).
pub struct SemphoreSmoker {
    papel_tabaco: Arc<Semaphore>,   // Semáforo para controlar el acceso a papel y tabaco.
    papel_fosforos: Arc<Semaphore>, // Semáforo para controlar el acceso a papel y fósforos.
    tabaco_fosforos: Arc<Semaphore>, // Semáforo para controlar el acceso a tabaco y fósforos.
    agente: Arc<Semaphore>,         // Semáforo para coordinar la notificación al agente.
}

impl SemphoreSmoker {
    /// Constructor que inicializa un nuevo sistema de sincronización.
    ///
    /// # Retorno
    /// Retorna una instancia de `SemphoreSmoker` con semáforos inicializados en 0 permisos.
    pub fn new() -> Self {
        Self {
            papel_tabaco: Arc::new(Semaphore::new(0)),   // Inicialmente, no hay combinaciones disponibles.
            papel_fosforos: Arc::new(Semaphore::new(0)), // Igual para papel y fósforos.
            tabaco_fosforos: Arc::new(Semaphore::new(0)), // Igual para tabaco y fósforos.
            agente: Arc::new(Semaphore::new(0)),         // Semáforo para el agente.
        }
    }

    /// Método para notificar que hay disponibles papel y tabaco.
    ///
    /// Incrementa el semáforo correspondiente para desbloquear a los fumadores con fósforos.
    pub async fn colocar_papel_tabaco(&self) {
        self.papel_tabaco.add_permits(1); // Añade un permiso para papel y tabaco.
    }

    /// Método para notificar que hay disponibles papel y fósforos.
    ///
    /// Incrementa el semáforo correspondiente para desbloquear a los fumadores con tabaco.
    pub async fn colocar_papel_fosforos(&self) {
        self.papel_fosforos.add_permits(1); // Añade un permiso para papel y fósforos.
    }

    /// Método para notificar que hay disponibles tabaco y fósforos.
    ///
    /// Incrementa el semáforo correspondiente para desbloquear a los fumadores con papel.
    pub async fn colocar_tabaco_fosforos(&self) {
        self.tabaco_fosforos.add_permits(1); // Añade un permiso para tabaco y fósforos.
    }

    /// Método para que un fumador con fósforos espere por papel y tabaco.
    ///
    /// Bloquea hasta que el semáforo correspondiente tenga permisos disponibles.
    pub async fn esperar_papel_tabaco(&self) {
        self.papel_tabaco.acquire().await.unwrap(); // Adquiere un permiso de papel y tabaco.
    }

    /// Método para que un fumador con tabaco espere por papel y fósforos.
    ///
    /// Bloquea hasta que el semáforo correspondiente tenga permisos disponibles.
    pub async fn esperar_papel_fosforos(&self) {
        self.papel_fosforos.acquire().await.unwrap(); // Adquiere un permiso de papel y fósforos.
    }

    /// Método para que un fumador con papel espere por tabaco y fósforos.
    ///
    /// Bloquea hasta que el semáforo correspondiente tenga permisos disponibles.
    pub async fn esperar_tabaco_fosforos(&self) {
        self.tabaco_fosforos.acquire().await.unwrap(); // Adquiere un permiso de tabaco y fósforos.
    }

    /// Método para que el agente espere a que un fumador termine de fumar.
    ///
    /// Bloquea al agente hasta que un fumador lo notifique.
    pub async fn esperar_fumador(&self) {
        self.agente.acquire().await.unwrap(); // Adquiere un permiso del semáforo del agente.
    }

    /// Método para notificar al agente que un fumador terminó de fumar.
    ///
    /// Incrementa el semáforo del agente para permitir que continúe el proceso.
    pub async fn notificar_agente(&self) {
        self.agente.add_permits(1); // Añade un permiso para notificar al agente.
    }
}
