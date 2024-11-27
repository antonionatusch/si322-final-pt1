use tokio::sync::{Semaphore};
use std::sync::Arc;

pub struct SemphoreSmoker {
    papel_tabaco: Arc<Semaphore>,
    papel_fosforos: Arc<Semaphore>,
    tabaco_fosforos: Arc<Semaphore>,
    agente: Arc<Semaphore>,
}

impl SemphoreSmoker {
    pub fn new() -> Self {
        Self {
            papel_tabaco: Arc::new(Semaphore::new(0)),
            papel_fosforos: Arc::new(Semaphore::new(0)),
            tabaco_fosforos: Arc::new(Semaphore::new(0)),
            agente: Arc::new(Semaphore::new(0)),
        }
    }

    pub async fn colocar_papel_tabaco(&self) {
        self.papel_tabaco.add_permits(1);
    }

    pub async fn colocar_papel_fosforos(&self) {
        self.papel_fosforos.add_permits(1);
    }

    pub async fn colocar_tabaco_fosforos(&self) {
        self.tabaco_fosforos.add_permits(1);
    }

    pub async fn esperar_papel_tabaco(&self) {
        self.papel_tabaco.acquire().await.unwrap();
    }

    pub async fn esperar_papel_fosforos(&self) {
        self.papel_fosforos.acquire().await.unwrap();
    }

    pub async fn esperar_tabaco_fosforos(&self) {
        self.tabaco_fosforos.acquire().await.unwrap();
    }

    pub async fn esperar_fumador(&self) {
        self.agente.acquire().await.unwrap();
    }

    pub async fn notificar_agente(&self) {
        self.agente.add_permits(1);
    }
}
