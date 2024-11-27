use std::sync::{Arc, Mutex};

mod buffer;
mod producer;
mod consumer;

use crate::buffer::Buffer;
use crate::producer::Producer;
use crate::consumer::Consumer;

const BUFFER_SIZE: usize = 5;
const PRODCUER_SIZE: i32 = 10;
const CONSUMER_SIZE: i32 = 10;

fn main() {
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
