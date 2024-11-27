use crate::estados::Estado;
use crate::semaforo_filosofos::Semaforo; // Importa el semáforo si lo usas
use std::sync::{Condvar, Mutex};

pub struct Mesa {
    pub estado: Mutex<[Estado; 5]>, // Estados de los filósofos
    pub condvar: [Condvar; 5],      // Condición para cada filósofo
    pub semaforo: Semaforo,         // Semáforo para limitar cuántos filósofos comen
}

impl Mesa {
    pub fn new(max_comiendo: usize) -> Self { // Acepta el parámetro max_comiendo
        Self {
            estado: Mutex::new([Estado::Pensando; 5]), // Inicializa los estados como Pensando
            condvar: Default::default(),               // Inicializa las variables de condición
            semaforo: Semaforo::new(max_comiendo),     // Inicializa el semáforo
        }
    }
}
impl Mesa {
    pub async fn tomar_tenedores(&self, i: usize) {
        // Lógica para tomar tenedores
        println!("Filósofo {} intenta tomar tenedores.", i);
    }

    pub fn dejar_tenedores(&self, i: usize) {
        // Lógica para dejar tenedores
        println!("Filósofo {} deja los tenedores.", i);
    }
}
