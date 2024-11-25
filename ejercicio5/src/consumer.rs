use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::buffer::Buffer;

pub struct Consumer {
    buffer: Arc<Mutex<Buffer>>,
}

impl Consumer {
    /// Crea una nueva instancia de la clase `Consumer`.
    /// 
    /// # Parámetros
    /// - `buffer`: un `Arc<Mutex<Buffer>>` que apunta al buffer compartido.
    /// 
    /// # Retorno
    /// Retorna una nueva instancia de `Consumer` asociada al buffer compartido.
    pub fn new(buffer: Arc<Mutex<Buffer>>) -> Self {
        Consumer { buffer }
    }

    /// Ejecuta el consumidor en un hilo separado.
    /// 
    /// @Arc::clone() se debe clonar para no mover toda la propiedad lo que lo haria inaccesible
    /// @lock().unwrap() espera hasta que el buffer tenga al menos un elemento y continua
    /// @remove() eliminar el primer valor del buffer liberando espacio para que el productor pueda seguir agregando nuevos elementos
    /// @sleep() simula tiempo
    /// 
    /// # Retorno
    /// Retorna un `JoinHandle<()>` que representa el hilo en el cual el consumidor está ejecutándose.
    pub fn run(&self) -> thread::JoinHandle<()> {
        let buffer = Arc::clone(&self.buffer);
        thread::spawn(move || {
            for _ in 1..=10 {
                loop {
                    {
                        let mut buf = buffer.lock().unwrap();
                        if !buf.is_empty() {
                            let value: i32 = buf.data.remove(0);
                            println!("Consumió: {}", value);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_millis(50));
                }
                thread::sleep(Duration::from_millis(400));
            }
        })
    }
}
