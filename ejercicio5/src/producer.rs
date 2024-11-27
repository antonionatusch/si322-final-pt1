use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::buffer::Buffer;

pub struct Producer {
    buffer: Arc<Mutex<Buffer>>,
    items_to_produce: i32,
}

impl Producer {
    /// Crea una nueva instancia de la clase `Producer`.
    /// 
    /// # Parámetros
    /// - `buffer`: un `Arc<Mutex<Buffer>>` que apunta al buffer compartido.
    /// - `items_to_produce`: número de elementos que el productor generará.
    /// 
    /// # Retorno
    /// Retorna una nueva instancia de `Producer` asociada al buffer compartido.
    pub fn new(buffer: Arc<Mutex<Buffer>>, items_to_produce: i32) -> Self {
        Producer { buffer, items_to_produce }
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
        let items = self.items_to_produce;
        thread::spawn(move || {
            for i in 1..=items {
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
