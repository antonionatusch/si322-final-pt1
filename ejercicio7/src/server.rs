use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};
use tokio::task;

const  MAX_CONNECTION: usize = 3;
const TOTAL_USERS: usize = 10;

/// Inicia la simulación de conexiones concurrentes al servidor.
///
/// Esta función simula un sistema donde hasta `MAX_CONNECTION` usuarios pueden conectarse al servidor 
/// al mismo tiempo. Los usuarios adicionales deben esperar a que se libere un permiso antes de continuar.
/// 
/// @Arc::new(Semaphore::new()) Se utiliza para inicializar un semáforo que controla el acceso simultáneo.
/// @task::spawn() Lanza tareas asíncronas que representan las conexiones de los usuarios.
pub async fn start_connection() {
    let semaphore = Arc::new(Semaphore::new(MAX_CONNECTION));
    
    let mut tasks = vec![];

    for i in 1..=TOTAL_USERS {
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

/// Maneja una conexión simulada para un usuario específico.
///
/// Representa el manejo de una conexión al servidor para un usuario individual.
/// Cada usuario intenta adquirir un permiso antes de iniciar la conexión.
///
/// @Arc::clone() Clona el semáforo compartido para que cada tarea lo utilice.
/// @semaphore.acquire() Adquiere un permiso del semáforo antes de manejar la conexión.
/// @sleep(Duration::from_secs(2)) Simula el tiempo necesario para procesar la conexión.
/// @await Libera el permiso automáticamente al salir del alcance.
///
/// # Parámetros
/// - `id`: Identificador único del usuario que realiza la conexión.
/// - `semaphore`: Semáforo compartido para controlar el número de conexiones activas.
async fn handle_connection(id: usize, semaphore: Arc<Semaphore>) {
    let _permit = semaphore.acquire().await.unwrap();

    println!("Conexión {} aceptada. ", id);
    
    sleep(Duration::from_secs(2)).await;

    println!("Conexión {} terminada.", id);
}