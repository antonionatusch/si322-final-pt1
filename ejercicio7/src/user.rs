use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::Semaphore;

// Clase para representar un usuario
// Permite duplicar el valor en lugar de moverlo
#[derive(Clone)]
struct User {
    id: usize,
    priority: usize,
}

// Clase para representar la gestión de impresoras
struct PrinterManager {
    semaphore: Arc<Semaphore>,
    priority_queue: Arc<Mutex<Vec<User>>>,
}

impl PrinterManager {
    // Crear un nuevo gestor de impresoras con un número fijo de permisos
    fn new(max_printers: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_printers)),
            priority_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Agregar un usuario a la cola de prioridad
    fn add_user(&self, user: User) {
        let mut queue = self.priority_queue.lock().unwrap();
        queue.push(user);
        queue.sort_by(|a, b| b.priority.cmp(&a.priority)); // Sort by descending priority
    }

    // Gestionar el acceso de un usuario a una impresora
    fn manage_access(&self, user: User) {
        loop {
            let permission_available = {
                let queue = self.priority_queue.lock().unwrap();
                if let Some(pos) = queue.iter().position(|u| u.id == user.id) {
                    pos < 2 // Allow if within the top two users with the highest priority
                } else {
                    false
                }
            };

            if permission_available {
                if let Ok(_permit) = self.semaphore.try_acquire() {
                    println!(
                        "Usuario {} (prioridad {}) ha adquirido impresora.",
                        user.id, user.priority
                    );

                    // Simulate work
                    thread::sleep(Duration::from_secs(2));

                    println!("Usuario {} ha liberado impresora.", user.id);

                    // Remove the user from the queue
                    let mut queue = self.priority_queue.lock().unwrap();
                    if let Some(pos) = queue.iter().position(|u| u.id == user.id) {
                        queue.remove(pos);
                    }
                    break;
                }
            } else {
                thread::sleep(Duration::from_millis(100)); // Wait briefly before trying again
            }
        }
    }
}


pub fn use_printer_with_priority() {
    let max_connection = 2;
    let total_user = 10;
    // Crear el gestor de impresoras con 2 impresoras disponibles
    let manager = Arc::new(PrinterManager::new(max_connection));

    // Crear usuarios con diferentes prioridades
    let mut users = Vec::new();
    for id in 1..=total_user {
        users.push(User {
            id,
            priority: 10 - id + 1, // Decreasing priorities
        });
    }
    

    let mut handles = vec![];

    // Crear hilos para cada usuario
    for user in users {
        let manager_clone = Arc::clone(&manager);
        handles.push(thread::spawn(move || {
            manager_clone.add_user(user.clone());
            manager_clone.manage_access(user);
        }));
    }

    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }
}
