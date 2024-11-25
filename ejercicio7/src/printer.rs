use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Semaphore;

pub fn use_printer() {
    // Crear un semáforo con 2 permisos disponibles
    let semaphore = Arc::new(Semaphore::new(2));

    // Crear un vector para manejar los hilos
    let mut handles = vec![];

    // Crear 10 hilos
    for i in 1..=10 {
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