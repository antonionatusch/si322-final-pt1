use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::buffer::Buffer;
use crate::producer::Producer;
use crate::consumer::Consumer;
use crate::monitor::ReaderWriterMonitor;

const BUFFER_SIZE: usize = 5;
const PRODCUER_SIZE: i32 = 10;
const CONSUMER_SIZE: i32 = 10;
const READER_SIZE: i32 = 5;
const WRITER_SIZE: i32 = 5;

/// Funcion del ejercicio 14
/// 
/// Para este problema se tiene un monito 
/// para los lectores y escritores
pub fn problema_14() {
        // Crear el buffer compartido
        let buffer = Arc::new(Mutex::new(Buffer::new(BUFFER_SIZE)));

        // Crear el productor y el consumidor
        let producer = Producer::new(Arc::clone(&buffer), PRODCUER_SIZE);
        let consumer = Consumer::new(Arc::clone(&buffer), CONSUMER_SIZE);
    
        // Ejecutar productor y consumidor
        let producer_handle = producer.run();
        let consumer_handle = consumer.run();
    
        // Esperar a que ambos hilos terminen
        producer_handle.join().unwrap();
        consumer_handle.join().unwrap();
}

/// Funcion del ejercicio 15
/// 
/// Para este problema se tiene un buffer compartido
/// para productor y consumidor
pub fn problema_15() {
    let monitor = Arc::new(ReaderWriterMonitor::new());

    // Crear hilos de lectores
    let mut handles = vec![];
    for i in 1..=READER_SIZE {
        let monitor_clone = Arc::clone(&monitor);
        handles.push(thread::spawn(move || {
            monitor_clone.start_read();
            println!("Lector {} está leyendo...", i);
            thread::sleep(Duration::from_secs(1));
            println!("Lector {} terminó de leer.", i);
            monitor_clone.end_read();
        }));
    }

    // Crear hilos de escritores
    for i in 1..=WRITER_SIZE {
        let monitor_clone = Arc::clone(&monitor);
        handles.push(thread::spawn(move || {
            monitor_clone.start_write();
            println!("Escritor {} está escribiendo...", i);
            thread::sleep(Duration::from_secs(2));
            println!("Escritor {} terminó de escribir.", i);
            monitor_clone.end_write();
        }));
    }

    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }
}