//! Módulo que encapsula las opciones del menú del sistema.
//!
//! Este módulo maneja múltiples problemas, incluyendo:
//! - El sistema productor-consumidor.
//! - Sincronización de procesos basada en secuencias específicas (Ejercicio 8).

use crate::buffer::Buffer;
use crate::consumer::Consumer;
use crate::producer::Producer;
use crate::semaphore::CustomSemaphore;
use crate::sequence_sync::SequenceSync; // Importar el módulo de sincronización de secuencias
use std::io;
use tokio::task;
/// Tamaño máximo del buffer compartido.
const BUFFER_SIZE: usize = 100;

/// Opción del menú que ejecuta el planteamiento inicial del problema productor-consumidor.
pub async fn problema_1_planteamiento_inicial() {
    let (buffer, empty_slots, full_slots) = initialize_shared_resources();
    let (producer, consumer) = initialize_producer_consumer(buffer, empty_slots, full_slots);
    execute_producer_consumer(producer, consumer).await;
}

/// Opción del menú que ejecuta el planteamiento corregido del problema productor-consumidor.
pub async fn problema_1_planteamiento_corregido() {
    let (buffer, empty_slots, full_slots) = initialize_shared_resources();
    let (producer, consumer) = initialize_producer_consumer(buffer, empty_slots, full_slots);
    execute_producer_consumer_corregido(producer, consumer).await;
}

/// Opción del menú que ejecuta la sincronización de secuencias del Ejercicio 8.
pub async fn problema_2_sincronizacion_de_secuencias() {
    let sync = SequenceSync::new();

    println!("Seleccione el caso:");
    println!("a) Secuencia ABCDEABCDE...");
    println!("b) Secuencia ACDEBACDEB...");
    println!("c) Secuencia (A o B)CDE...");
    println!("d) Secuencia (A o B)CE(A o B)DE...");

    let mut case = String::new();
    io::stdin()
        .read_line(&mut case)
        .expect("Error al leer entrada");

    match case.trim() {
        "a" => sync.case_a().await,
        "b" => sync.case_b().await,
        "c" => sync.case_c().await,
        "d" => sync.case_d().await,
        _ => println!("Opción no válida"),
    }
}

/// Inicializa el buffer compartido y los semáforos para sincronización.
fn initialize_shared_resources() -> (Buffer, CustomSemaphore, CustomSemaphore) {
    let buffer = Buffer::new(BUFFER_SIZE);
    let empty_slots = CustomSemaphore::new(BUFFER_SIZE);
    let full_slots = CustomSemaphore::new(0);

    (buffer, empty_slots, full_slots)
}

/// Inicializa el productor y el consumidor.
fn initialize_producer_consumer(
    buffer: Buffer,
    empty_slots: CustomSemaphore,
    full_slots: CustomSemaphore,
) -> (Producer, Consumer) {
    let producer = Producer::new(buffer.clone(), empty_slots.clone(), full_slots.clone());
    let consumer = Consumer::new(buffer, empty_slots, full_slots);

    (producer, consumer)
}

/// Ejecuta las tareas asincrónicas del productor y el consumidor.
async fn execute_producer_consumer(producer: Producer, consumer: Consumer) {
    let producer_handle = task::spawn(async move {
        producer.produce().await;
    });

    let consumer_handle = task::spawn(async move {
        consumer.consume().await;
    });

    let _ = tokio::join!(producer_handle, consumer_handle);
}

/// Ejecuta las tareas asincrónicas del productor y el consumidor con corrección.
async fn execute_producer_consumer_corregido(producer: Producer, consumer: Consumer) {
    let producer_handle = task::spawn(async move {
        producer.produce_corregido().await;
    });

    let consumer_handle = task::spawn(async move {
        consumer.consume_corregido().await;
    });

    let _ = tokio::join!(producer_handle, consumer_handle);
}
