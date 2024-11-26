use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{sleep, Duration};

/// Representa el tarro de miel compartido.
pub struct HoneyJar {
    capacity: usize,
    current: Mutex<usize>,
    bear_ready: Semaphore,
}

impl HoneyJar {
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            capacity,
            current: Mutex::new(0),
            bear_ready: Semaphore::new(0), // Inicialmente el oso duerme.
        })
    }

    /// Incrementa la cantidad de miel en el tarro.
    pub async fn add_honey(&self, bee_id: usize) -> bool {
        let mut honey = self.current.lock().await;
        *honey += 1;
        println!(
            "Abeja {} añadió una porción de miel. Miel actual: {}/{}",
            bee_id, *honey, self.capacity
        );

        if *honey >= self.capacity {
            println!("El tarro está lleno. Llamando al oso...");
            self.bear_ready.add_permits(1); // Despierta al oso.
            return true;
        }
        false
    }

    /// Consume toda la miel del tarro.
    pub async fn consume_all(&self) {
        self.bear_ready.acquire().await; // Espera hasta que el tarro esté lleno.
        let mut honey = self.current.lock().await;
        println!("El oso se despierta y consume toda la miel.");
        *honey = 0; // El tarro queda vacío.
        println!("El oso vuelve a dormir. El tarro está vacío.");
    }
}

/// Representa una abeja productora.
pub async fn bee_task(honey_jar: Arc<HoneyJar>, bee_id: usize) {
    loop {
        if honey_jar.add_honey(bee_id).await {
            sleep(Duration::from_millis(1000)).await; // Simula tiempo entre llamadas.
        }
    }
}

/// Representa la tarea del oso.
pub async fn bear_task(honey_jar: Arc<HoneyJar>) {
    loop {
        honey_jar.consume_all().await;
        sleep(Duration::from_secs(1)).await; // Simula el tiempo que el oso duerme.
    }
}
