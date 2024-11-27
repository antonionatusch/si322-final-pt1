use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct SemaforOperarios {
    ruedas: Arc<Semaphore>,
    cuadros: Arc<Semaphore>,
    manillares: Arc<Semaphore>,
}

impl SemaforOperarios {
    pub fn new() -> Self {
        Self {
            ruedas: Arc::new(Semaphore::new(0)),     // Sin ruedas iniciales
            cuadros: Arc::new(Semaphore::new(0)),    // Sin cuadros iniciales
            manillares: Arc::new(Semaphore::new(0)), // Sin manillares iniciales
        }
    }

    pub async fn rueda_producida(&self) {
        self.ruedas.add_permits(1); // Añade una rueda
    }

    pub async fn cuadro_producido(&self) {
        self.cuadros.add_permits(1); // Añade un cuadro
    }

    pub async fn manillar_producido(&self) {
        self.manillares.add_permits(1); // Añade un manillar
    }

    pub async fn esperar_piezas(&self) {
        // Espera por dos ruedas
        self.ruedas.acquire().await.unwrap();
        self.ruedas.acquire().await.unwrap();

        // Espera por un cuadro
        self.cuadros.acquire().await.unwrap();

        // Espera por un manillar
        self.manillares.acquire().await.unwrap();
    }
}
