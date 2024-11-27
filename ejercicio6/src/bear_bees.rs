use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{sleep, Duration};

/// Representa el tarro de miel compartido.
pub struct HoneyJar {
    capacity: usize,
    current: Mutex<usize>,
    bees_allowed: Semaphore, // Controla si las abejas pueden producir.
    bear_active: Semaphore,  // Controla si el oso está activo (pausa la producción de abejas).
}

impl HoneyJar {
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            capacity,
            current: Mutex::new(0),
            bees_allowed: Semaphore::new(capacity), // Limita a las abejas para evitar sobrellenar el tarro.
            bear_active: Semaphore::new(1), // Las abejas trabajan mientras el oso no está activo.
        })
    }

    /// Incrementa la cantidad de miel en el tarro y llama al oso si está lleno.
    pub async fn add_honey(&self, bee_id: usize) {
        // Esperar a que el oso no esté activo.
        let _bear_inactive = self.bear_active.acquire().await.unwrap();
        // Esperar a que haya espacio en el tarro.
        let _permit = self.bees_allowed.acquire().await.unwrap();

        let mut honey = self.current.lock().await;
        *honey += 1;
        println!(
            "Abeja {} añadió una porción de miel. Miel actual: {}/{}",
            bee_id, *honey, self.capacity
        );

        // Si el tarro está lleno, el oso consume la miel inmediatamente.
        if *honey >= self.capacity {
            println!("El tarro está lleno. El oso consume la miel...");
            *honey = 0; // Vacía el tarro.
            println!("El oso vuelve a dormir. El tarro está vacío.");

            // Permitir que las abejas vuelvan a llenar el tarro.
            self.bees_allowed.add_permits(self.capacity);
        }

        self.bear_active.add_permits(1); // Permitir que las abejas continúen.
    }
}

/// Representa una abeja productora.
pub async fn bee_task(honey_jar: Arc<HoneyJar>, bee_id: usize) {
    loop {
        honey_jar.add_honey(bee_id).await;
        sleep(Duration::from_millis(500)).await; // Simula el tiempo de producción.
    }
}
