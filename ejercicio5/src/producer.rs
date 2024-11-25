use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::buffer::Buffer;

pub struct Producer {
    buffer: Arc<Mutex<Buffer>>,
}

impl Producer {
    /// Crea una nueva instancia de la clase `Producer`.
    /// 
    /// # Parámetros
    /// - `buffer`: un `Arc<Mutex<Buffer>>` que apunta al buffer compartido.
    /// 
    /// # Retorno
    /// Retorna una nueva instancia de `Producer` asociada al buffer compartido.
    pub fn new(buffer: Arc<Mutex<Buffer>>) -> Self {
        Producer { buffer }
    }

    /// Ejecuta el productor en un hilo separado.
    /// 
    /// @Arc::clone() se debe clonar para no mover toda la propiedad lo que lo haria inaccesible
    /// @lock().unwrap() espera hasta que el buffer tenga espacio disponible y continua 
    /// @sleep() simula tiempo
    /// 
    /// # Retorno
    /// Retorna un `JoinHandle<()>` que representa el hilo en el cual el productor está ejecutándose.
    pub fn run(&self) -> thread::JoinHandle<()> {
        let buffer_clone = Arc::clone(&self.buffer);
        thread::spawn(move || {
            for i in 1..=10 {
                loop {
                    {
                        let mut buf = buffer_clone.lock().unwrap();
                        if !buf.is_full() {
                            buf.data.push(i);
                            println!("Produjo: {}", i);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_millis(50));
                }
                thread::sleep(Duration::from_millis(100));
            }
        })
    }
}
