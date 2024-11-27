use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct Semaforo {
    semaforo: Arc<Semaphore>,
}

impl Semaforo {
    pub fn new(max_permits: usize) -> Self { // Constructor acepta max_permits
        Self {
            semaforo: Arc::new(Semaphore::new(max_permits)),
        }
    }

    pub async fn acquire(&self) {
        self.semaforo.acquire().await.unwrap();
    }

    pub fn release(&self) {
        self.semaforo.add_permits(1);
    }
}
