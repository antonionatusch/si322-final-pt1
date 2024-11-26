use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Semaphore;

const MAX_CONNECTIONS: usize = 2;
const TOTAL_USERS: i32 = 10;

/// Simula el acceso concurrente a una impresora compartida por múltiples usuarios.
///
/// Esta función utiliza un semáforo para limitar el acceso simultáneo a un recurso (impresora) 
/// a un número máximo de usuarios (`MAX_CONNECTIONS`). Los usuarios adicionales deben esperar
/// a que se liberen permisos antes de poder acceder al recurso.
///
/// @Arc::new(Semaphore::new()) Inicializa un semáforo con un número fijo de permisos.
/// @Arc::clone() Clona la referencia compartida al semáforo para los hilos.
/// @thread::spawn() Crea múltiples hilos para simular usuarios concurrentes.
/// @try_acquire() Intenta adquirir un permiso del semáforo sin bloquear indefinidamente.
/// @thread::sleep() Simula el tiempo necesario para usar el recurso y libera el permiso automáticamente.
pub fn use_printer() {
    // Crear un semáforo con 2 permisos disponibles
    let semaphore = Arc::new(Semaphore::new(MAX_CONNECTIONS));

    // Crear un vector para manejar los hilos
    let mut handles = vec![];

    // Crear 10 hilos
    for i in 1..=TOTAL_USERS {
        let sem_clone = Arc::clone(&semaphore);
        let handle = thread::spawn(move || {
            loop {
                if let Ok(_permit) = sem_clone.try_acquire() {
                    println!("Usuario (hilo) {} ha adquirido un permiso.", i);

                    // Simular trabajo (recurso en uso)
                    thread::sleep(Duration::from_secs(2));

                    // El permiso se libera automáticamente cuando `permit` sale del alcance
                    println!("Usuario {} ha liberado el permiso.", i);
                    break; // Salir del bucle después de adquirir el permiso
                } else {
                    // Esperar brevemente antes de intentar nuevamente
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
        handles.push(handle);
    }

    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }
}