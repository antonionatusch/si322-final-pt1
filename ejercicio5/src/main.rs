use std::sync::{Arc, Mutex};

mod buffer;
mod producer;
mod consumer;

use crate::buffer::Buffer;
use crate::producer::Producer;
use crate::consumer::Consumer;

const BUFFER_SIZE: usize = 5;

fn main() {
    // Crear el buffer compartido
    let buffer = Arc::new(Mutex::new(Buffer::new(BUFFER_SIZE)));

    // Crear el productor y el consumidor
    let producer = Producer::new(Arc::clone(&buffer));
    let consumer = Consumer::new(Arc::clone(&buffer));

    // Ejecutar productor y consumidor
    let producer_handle = producer.run();
    let consumer_handle = consumer.run();

    // Esperar a que ambos hilos terminen
    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
