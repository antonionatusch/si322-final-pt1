//! Implementación del manejo de sincronización de secuencias (Ejercicio 8).

use std::sync::Arc;
use tokio::sync::Semaphore;

/// Estructura para manejar la sincronización de procesos en el Ejercicio 8.
#[derive(Clone)]
pub struct SequenceSync {
    semaphore_a: Arc<Semaphore>,
    semaphore_b: Arc<Semaphore>,
    semaphore_c: Arc<Semaphore>,
    semaphore_d: Arc<Semaphore>,
    semaphore_e: Arc<Semaphore>,
}

impl SequenceSync {
    /// Crea una nueva instancia para manejar la sincronización.
    pub fn new() -> Self {
        Self {
            semaphore_a: Arc::new(Semaphore::new(1)), // A puede iniciar
            semaphore_b: Arc::new(Semaphore::new(0)), // B espera a A
            semaphore_c: Arc::new(Semaphore::new(0)), // C espera a B
            semaphore_d: Arc::new(Semaphore::new(0)), // D espera a C
            semaphore_e: Arc::new(Semaphore::new(0)), // E espera a D
        }
    }

    /// Ejecuta las tareas según el caso (a).
    pub async fn case_a(&self) {
        loop {
            let _ = self.semaphore_a.acquire().await;
            println!("A ejecutado");
            self.semaphore_b.add_permits(1); // Desbloquea B

            let _ = self.semaphore_b.acquire().await;
            println!("B ejecutado");
            self.semaphore_c.add_permits(1); // Desbloquea C

            let _ = self.semaphore_c.acquire().await;
            println!("C ejecutado");
            self.semaphore_d.add_permits(1); // Desbloquea D

            let _ = self.semaphore_d.acquire().await;
            println!("D ejecutado");
            self.semaphore_e.add_permits(1); // Desbloquea E

            let _ = self.semaphore_e.acquire().await;
            println!("E ejecutado");
            self.semaphore_a.add_permits(1); // Reinicia el ciclo
        }
    }

    /// Ejecuta las tareas según el caso (b).
    pub async fn case_b(&self) {
        loop {
            let _ = self.semaphore_a.acquire().await;
            println!("A ejecutado");
            self.semaphore_c.add_permits(1); // Desbloquea C

            let _ = self.semaphore_c.acquire().await;
            println!("C ejecutado");
            self.semaphore_b.add_permits(1); // Desbloquea B

            let _ = self.semaphore_b.acquire().await;
            println!("B ejecutado");
            self.semaphore_d.add_permits(1); // Desbloquea D

            let _ = self.semaphore_d.acquire().await;
            println!("D ejecutado");
            self.semaphore_e.add_permits(1); // Desbloquea E

            let _ = self.semaphore_e.acquire().await;
            println!("E ejecutado");
            self.semaphore_a.add_permits(1); // Reinicia el ciclo
        }
    }

    /// Ejecuta las tareas según el caso (c) corregido.
    pub async fn case_c(&self) {
        let mut alternate = true; // Variable para alternar entre A y B
        let mut step = 0; // Rastrea la etapa actual en la secuencia

        loop {
            match step {
                0 => {
                    // Primera ejecución de A o B
                    if alternate {
                        let _ = self.semaphore_a.acquire().await;
                        println!("A ejecutado");
                    } else {
                        let _ = self.semaphore_b.acquire().await;
                        println!("B ejecutado");
                    }
                    alternate = !alternate; // Alterna entre A y B
                    self.semaphore_c.add_permits(1); // Desbloquea C
                    step = 1;
                }
                1 => {
                    // Ejecución de C
                    let _ = self.semaphore_c.acquire().await;
                    println!("C ejecutado");
                    self.semaphore_d.add_permits(1); // Desbloquea D
                    step = 2;
                }
                2 => {
                    // Ejecución de D
                    let _ = self.semaphore_d.acquire().await;
                    println!("D ejecutado");
                    self.semaphore_e.add_permits(1); // Desbloquea E
                    step = 3;
                }
                3 => {
                    // Ejecución de E
                    let _ = self.semaphore_e.acquire().await;
                    println!("E ejecutado");
                    self.semaphore_a.add_permits(1); // Reinicia el ciclo para A/B
                    self.semaphore_b.add_permits(1); // Reinicia el ciclo para A/B
                    step = 0; // Reinicia el ciclo
                }
                _ => panic!("Estado inválido"), // Evita estados inesperados
            }
        }
    }

    /// Ejecuta las tareas según el caso (d) corregido.
    pub async fn case_d(&self) {
        let mut alternate = true; // Alternar entre A y B
        let mut step = 0; // Rastrea la etapa actual en la secuencia

        loop {
            match step {
                0 => {
                    if alternate {
                        let _ = self.semaphore_a.acquire().await;
                        println!("A ejecutado");
                    } else {
                        let _ = self.semaphore_b.acquire().await;
                        println!("B ejecutado");
                    }
                    alternate = !alternate; // Alterna entre A y B
                    self.semaphore_c.add_permits(1); // Desbloquea C
                    step = 1;
                }
                1 => {
                    let _ = self.semaphore_c.acquire().await;
                    println!("C ejecutado");
                    self.semaphore_e.add_permits(1); // Desbloquea E
                    step = 2;
                }
                2 => {
                    let _ = self.semaphore_e.acquire().await;
                    println!("E ejecutado");
                    self.semaphore_a.add_permits(1); // Alterna entre A y B
                    self.semaphore_b.add_permits(1); // Alterna entre A y B
                    step = 3;
                }
                3 => {
                    if alternate {
                        let _ = self.semaphore_a.acquire().await;
                        println!("A ejecutado");
                    } else {
                        let _ = self.semaphore_b.acquire().await;
                        println!("B ejecutado");
                    }
                    alternate = !alternate; // Alterna entre A y B
                    step = 4; // Continúa con la segunda ejecución de A o B
                }
                4 => {
                    if alternate {
                        let _ = self.semaphore_a.acquire().await;
                        println!("A ejecutado");
                    } else {
                        let _ = self.semaphore_b.acquire().await;
                        println!("B ejecutado");
                    }
                    alternate = !alternate; // Alterna entre A y B
                    self.semaphore_d.add_permits(1); // Desbloquea D
                    step = 5;
                }
                5 => {
                    let _ = self.semaphore_d.acquire().await;
                    println!("D ejecutado");
                    self.semaphore_e.add_permits(1); // Desbloquea E
                    step = 6;
                }
                6 => {
                    let _ = self.semaphore_e.acquire().await;
                    println!("E ejecutado");
                    self.semaphore_a.add_permits(1); // Reinicia el ciclo
                    self.semaphore_b.add_permits(1); // Reinicia el ciclo
                    step = 0; // Reinicia el ciclo
                }
                _ => panic!("Estado inválido"), // Evita estados inesperados
            }
        }
    }
}
