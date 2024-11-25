use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::Semaphore;

// Clase para representar un usuario
// Permite duplicar el valor en lugar de moverlo
#[derive(Clone)]
struct Usuario {
    id: usize,
    prioridad: usize,
}

// Clase para representar la gestión de impresoras
struct GestorImpresoras {
    semaforo: Arc<Semaphore>,
    cola_prioridad: Arc<Mutex<Vec<Usuario>>>,
}

impl GestorImpresoras {
    // Crear un nuevo gestor de impresoras con un número fijo de permisos
    fn new(max_impresoras: usize) -> Self {
        Self {
            semaforo: Arc::new(Semaphore::new(max_impresoras)),
            cola_prioridad: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Agregar un usuario a la cola de prioridad
    fn agregar_usuario(&self, usuario: Usuario) {
        let mut cola = self.cola_prioridad.lock().unwrap();
        cola.push(usuario);
        cola.sort_by(|a, b| b.prioridad.cmp(&a.prioridad)); // Ordenar por prioridad descendente
    }

    // Gestionar el acceso de un usuario a una impresora
    fn gestionar_acceso(&self, usuario: Usuario) {
        loop {
            let permiso_disponible = {
                let cola = self.cola_prioridad.lock().unwrap();
                if let Some(pos) = cola.iter().position(|u| u.id == usuario.id) {
                    pos < 2 // Permitir si está entre los primeros dos usuarios con mayor prioridad
                } else {
                    false
                }
            };

            if permiso_disponible {
                if let Ok(_permiso) = self.semaforo.try_acquire() {
                    println!(
                        "Usuario {} (prioridad {}) ha adquirido una impresora.",
                        usuario.id, usuario.prioridad
                    );

                    // Simular trabajo
                    thread::sleep(Duration::from_secs(2));

                    println!("Usuario {} ha liberado la impresora.", usuario.id);

                    // Remover al usuario de la cola
                    let mut cola = self.cola_prioridad.lock().unwrap();
                    if let Some(pos) = cola.iter().position(|u| u.id == usuario.id) {
                        cola.remove(pos);
                    }
                    break;
                }
            } else {
                thread::sleep(Duration::from_millis(100)); // Esperar brevemente antes de intentar de nuevo
            }
        }
    }
}


pub fn use_printer_with_priority() {
    // Crear el gestor de impresoras con 2 impresoras disponibles
    let gestor = Arc::new(GestorImpresoras::new(2));

    // Crear usuarios con diferentes prioridades
    let mut usuarios = Vec::new();
    for id in 1..=5 {
        usuarios.push(Usuario {
            id,
            prioridad: 10 - id + 1, // Prioridades decrecientes
        });
    }
    

    let mut handles = vec![];

    // Crear hilos para cada usuario
    for usuario in usuarios {
        let gestor_clone = Arc::clone(&gestor);
        handles.push(thread::spawn(move || {
            gestor_clone.agregar_usuario(usuario.clone());
            gestor_clone.gestionar_acceso(usuario);
        }));
    }

    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }

}
