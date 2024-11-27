use crate::estados::Estado; // Importa el módulo que define los estados de los filósofos.
use crate::semaforo_filosofos::Semaforo; // Importa el semáforo para limitar cuántos filósofos comen simultáneamente.
use std::sync::{Condvar, Mutex};

/// Estructura que representa la mesa compartida por los filósofos.
///
/// La mesa incluye:
/// - Los estados de los filósofos.
/// - Variables de condición para la sincronización.
/// - Un semáforo para limitar cuántos filósofos pueden comer simultáneamente.
pub struct Mesa {
    pub estado: Mutex<[Estado; 5]>, // Estados de los filósofos (Pensando, Hambriento, Comiendo).
    pub condvar: [Condvar; 5],      // Variables de condición para manejar la espera de los filósofos.
    pub semaforo: Semaforo,         // Semáforo para controlar cuántos filósofos pueden comer.
}

impl Mesa {
    /// Constructor que crea una nueva mesa.
    ///
    /// # Parámetros
    /// - `max_comiendo`: Máximo número de filósofos que pueden comer simultáneamente.
    ///
    /// # Retorno
    /// Retorna una instancia de `Mesa` inicializada con:
    /// - Todos los filósofos en estado `Pensando`.
    /// - Variables de condición predeterminadas.
    /// - Un semáforo configurado con el valor de `max_comiendo`.
    pub fn new(max_comiendo: usize) -> Self {
        Self {
            estado: Mutex::new([Estado::Pensando; 5]), // Todos los filósofos empiezan pensando.
            condvar: Default::default(),               // Variables de condición inicializadas por defecto.
            semaforo: Semaforo::new(max_comiendo),     // Semáforo configurado con el límite de comensales.
        }
    }
}

impl Mesa {
    /// Función asincrónica para tomar tenedores.
    ///
    /// # Parámetros
    /// - `i`: Índice del filósofo que intenta tomar los tenedores.
    ///
    /// # Comportamiento
    /// - Un filósofo debe esperar si no puede comer (debido a disponibilidad de tenedores o límite de comensales).
    /// - Una vez que puede comer, actualiza su estado y notifica.
    pub async fn tomar_tenedores(&self, i: usize) {
        println!("Filósofo {} intenta tomar tenedores.", i);
        // Aquí se agregarían las operaciones de sincronización (Condvar y Semáforo).
    }

    /// Función para dejar los tenedores.
    ///
    /// # Parámetros
    /// - `i`: Índice del filósofo que deja los tenedores.
    ///
    /// # Comportamiento
    /// - Libera los tenedores ocupados por el filósofo.
    /// - Actualiza su estado y notifica a los filósofos adyacentes si pueden comer.
    pub fn dejar_tenedores(&self, i: usize) {
        println!("Filósofo {} deja los tenedores.", i);
        // Aquí se agregarían las operaciones para liberar los recursos y notificar a los vecinos.
    }
}
