use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};
use tokio::task;

pub async fn start_connection() {
    // Número máximo de conexiones permitidas
    let max_connections = 3;
    let semaphore = Arc::new(Semaphore::new(max_connections));
    
    // Simulamos múltiples clientes tratando de conectarse al servidor
    let mut tasks = vec![];
    for i in 1..=10 {
        let sem_clone = Arc::clone(&semaphore);
        tasks.push(task::spawn(async move {
            handle_connection(i, sem_clone).await;
        }));
    }
    
    // Esperar a que todas las tareas terminen
    for t in tasks {
        t.await.unwrap();
    }
}

async fn handle_connection(id: usize, semaphore: Arc<Semaphore>) {
    // Intentar adquirir un permiso del semáforo
    let _permit = semaphore.acquire().await.unwrap();
    // println!("Permisos disponibles: {}", semaphore.available_permits());
    println!("Conexión {} aceptada. ", id);
    
    // Simular manejo de conexión
    sleep(Duration::from_secs(2)).await;

    println!("Conexión {} terminada.", id);
    // El permiso se libera automáticamente al salir del alcance
}