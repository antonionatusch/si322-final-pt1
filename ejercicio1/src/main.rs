mod buffer;
mod consumer;
mod producer;
mod semaphore;

use buffer::Buffer;
use consumer::Consumer;
use producer::Producer;
use semaphore::CustomSemaphore;
use tokio::task;

/// Tamaño máximo del buffer compartido.
const BUFFER_SIZE: usize = 100;

#[tokio::main]
async fn main() {
    // Inicializar los componentes principales
    let (buffer, empty_slots, full_slots) = initialize_shared_resources();

    // Inicializar el productor y el consumidor
    let (producer, consumer) = initialize_producer_consumer(buffer, empty_slots, full_slots);

    // Ejecutar el sistema de productor-consumidor
    execute_producer_consumer(producer, consumer).await;
}

/// Inicializa el buffer compartido y los semáforos para sincronización.
///
/// # Retorno
/// Retorna una tupla con:
/// - El buffer compartido.
/// - El semáforo para espacios vacíos.
/// - El semáforo para espacios llenos.
fn initialize_shared_resources() -> (Buffer, CustomSemaphore, CustomSemaphore) {
    let buffer = Buffer::new(BUFFER_SIZE);
    let empty_slots = CustomSemaphore::new(BUFFER_SIZE);
    let full_slots = CustomSemaphore::new(0);

    (buffer, empty_slots, full_slots)
}

/// Inicializa el productor y el consumidor.
///
/// # Parámetros
/// - `buffer`: El buffer compartido.
/// - `empty_slots`: El semáforo para espacios vacíos.
/// - `full_slots`: El semáforo para espacios llenos.
///
/// # Retorno
/// Retorna una tupla con:
/// - El productor.
/// - El consumidor.
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
///
/// # Parámetros
/// - `producer`: El productor.
/// - `consumer`: El consumidor.
///
/// # Nota
/// Esta función espera que ambas tareas finalicen (siempre activas en este caso).
async fn execute_producer_consumer(producer: Producer, consumer: Consumer) {
    let producer_handle = task::spawn(async move {
        producer.produce().await;
    });

    let consumer_handle = task::spawn(async move {
        consumer.consume().await;
    });

    let _ = tokio::join!(producer_handle, consumer_handle);
}
