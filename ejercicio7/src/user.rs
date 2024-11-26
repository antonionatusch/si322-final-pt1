use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::Semaphore;

// Permite duplicar el valor en lugar de moverlo
#[derive(Clone)]
struct User {
    id: usize,
    priority: usize,
}

struct PrinterManager {
    semaphore: Arc<Semaphore>,
    priority_queue: Arc<Mutex<Vec<User>>>,
}

impl PrinterManager {
    /// Crea una nueva instancia de `PrinterManager`.
    ///
    /// # Parámetros
    /// - `max_printers`: Número máximo de impresoras disponibles.
    ///
    /// # Retorno
    /// Retorna una instancia de `PrinterManager` con un semáforo inicializado y una cola de prioridad vacía.
    ///
    /// @Arc::new(Semaphore::new()) Inicializa el semáforo con el número máximo de permisos.
    /// @Mutex<Vec<User>> Protege la cola de prioridad para acceso seguro entre hilos.
    fn new(max_printers: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_printers)),
            priority_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Agrega un usuario a la cola de prioridad.
    ///
    /// Este método inserta un usuario en la cola de prioridad y reorganiza la cola
    /// para que los usuarios con mayor prioridad aparezcan al principio.
    ///
    /// # Parámetros
    /// - `user`: Instancia del usuario a agregar.
    ///
    /// @priority_queue.push(user) Inserta al usuario en la cola.
    /// @priority_queue.sort_by() Ordena la cola en orden descendente de prioridades.
    fn add_user(&self, user: User) {
        let mut queue = self.priority_queue.lock().unwrap();
        queue.push(user);
        queue.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Gestiona el acceso de un usuario a una impresora.
    ///
    /// Este método permite que un usuario acceda a una impresora si:
    /// 1. Tiene una de las dos prioridades más altas en la cola.
    /// 2. Hay una impresora disponible.
    ///
    /// # Parámetros
    /// - `user`: Instancia del usuario que intenta acceder.
    ///
    /// @priority_queue.lock() Protege la cola de prioridad para verificar la posición del usuario.
    /// @semaphore.try_acquire() Adquiere un permiso del semáforo si hay impresoras disponibles.
    /// @thread::sleep() Simula el tiempo que el usuario utiliza la impresora.
    fn manage_access(&self, user: User) {
        loop {
            let permission_available = {
                let queue = self.priority_queue.lock().unwrap();
                if let Some(pos) = queue.iter().position(|u| u.id == user.id) {
                    pos < 2
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

                    thread::sleep(Duration::from_secs(2));

                    println!("Usuario {} ha liberado impresora.", user.id);

                    let mut queue = self.priority_queue.lock().unwrap();
                    if let Some(pos) = queue.iter().position(|u| u.id == user.id) {
                        queue.remove(pos);
                    }
                    break;
                }
            } else {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

/// Simula el acceso de múltiples usuarios a impresoras compartidas con manejo de prioridades.
///
/// La función crea usuarios con prioridades asignadas y utiliza múltiples hilos
/// para simular su acceso a impresoras compartidas. El acceso es gestionado por `PrinterManager`,
/// que asegura que los usuarios con mayor prioridad tengan acceso preferencial.
///
/// @thread::spawn() Crea un hilo por usuario.
/// @PrinterManager.add_user() Agrega al usuario a la cola de prioridad.
/// @PrinterManager.manage_access() Gestiona el acceso del usuario a la impresora.
pub fn use_printer_with_priority() {
    let max_connection = 2;
    let total_user = 10;

    let manager = Arc::new(PrinterManager::new(max_connection));

    let mut users = Vec::new();
    for id in 1..=total_user {
        users.push(User {
            id,
            priority: 10 - id + 1,
        });
    }
    
    let mut handles = vec![];

    for user in users {
        let manager_clone = Arc::clone(&manager);
        handles.push(thread::spawn(move || {
            manager_clone.add_user(user.clone());
            manager_clone.manage_access(user);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
