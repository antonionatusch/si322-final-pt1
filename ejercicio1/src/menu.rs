//! Módulo que encapsula las opciones del menú del sistema.
//!
//! Este módulo maneja múltiples problemas, incluyendo:
//! - El sistema productor-consumidor.
//! - Sincronización de procesos basada en secuencias específicas (Ejercicio 8).

use crate::buffer::Buffer;
use crate::consumer::Consumer;
use crate::producer::Producer;
use crate::semaphore::CustomSemaphore;
use tokio::task;

/// Tamaño máximo del buffer compartido.
const BUFFER_SIZE: usize = 100;

/// Opción del menú que ejecuta el planteamiento inicial del problema productor-consumidor.
///
/// # Nota
/// Esta implementación tiene problemas que pueden surgir debido a un mal manejo de la concurrencia:
/// - **Falta de manejo explícito de exclusión mutua**: Aunque el `Mutex` protege el buffer, no hay control explícito sobre
///   la exclusión mutua entre el productor y el consumidor.
/// - **Riesgo de condiciones límite**:
///   - El productor podría intentar producir cuando el buffer está lleno.
///   - El consumidor podría intentar consumir cuando el buffer está vacío.
/// - **Falta de validación de estado**: No hay forma de monitorear correctamente el estado del buffer y sincronizar
///   estrictamente al productor y al consumidor.
pub async fn problema_1_planteamiento_inicial() {
    // Inicializar los componentes principales
    let (buffer, empty_slots, full_slots) = initialize_shared_resources();

    // Inicializar el productor y el consumidor
    let (producer, consumer) = initialize_producer_consumer(buffer, empty_slots, full_slots);

    // Ejecutar el sistema de productor-consumidor
    execute_producer_consumer(producer, consumer).await;
}

/// Opción futura para ejecutar el planteamiento correcto del problema productor-consumidor.
///
/// # Nota
/// Esta función está pendiente de implementación.
/// - Se deberá corregir el mal manejo de concurrencia.
/// - Implementar validaciones estrictas para garantizar exclusión mutua y sincronización correcta.
pub async fn problema_1_planteamiento_correcto() {
    println!(
        "El planteamiento correcto del problema productor-consumidor aún no está implementado."
    );
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

/// Opción futura para sincronización de secuencias (Ejercicio 8).
///
/// # Nota
/// Esta función aún no está implementada.
pub async fn problema_2_sincronizacion_de_secuencias() {
    println!("Sincronización de secuencias (Ejercicio 8) aún no está implementada.");
}
