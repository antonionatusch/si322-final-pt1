mod fumadores; // Módulo para fumadores
mod semaforo_fumadores; // Módulo para sincronización de fumadores
mod agente;

mod operarios; // Módulo para operarios
mod montador; // Módulo para montadores
mod semaforo_operarios; // Módulo para sincronización de montadores

use std::sync::Arc;
use tokio::sync::Notify;
use std::io::{self, Write};
use std::time::Duration;

// Variable global para el número de iteraciones
const NUMERO_ITERACIONES: usize = 5;

#[tokio::main]
async fn main() {
    loop {
        println!("--- Menú Principal ---");
        println!("1. Ejecutar ejercicio de los fumadores de cigarrillos");
        println!("2. Ejecutar ejercicio de los montadores de bicicletas");
        println!("3. Salir");
        print!("Seleccione una opción: ");
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();
        let opcion = opcion.trim();

        match opcion {
            "1" => {
                println!("Iniciando el ejercicio de los fumadores de cigarrillos...");
                ejecutar_fumadores().await;
            }
            "2" => {
                println!("Iniciando el ejercicio de los montadores de bicicletas...");
                ejecutar_montadores().await;
            }
            "3" => {
                println!("Saliendo del programa...");
                break;
            }
            _ => {
                println!("Opción inválida. Intente nuevamente.");
            }
        }
    }
}

// Función para ejecutar el ejercicio de los fumadores
async fn ejecutar_fumadores() {
    let sincronizacion: Arc<semaforo_fumadores::SemphoreSmoker> = Arc::new(semaforo_fumadores::SemphoreSmoker::new());
    let notify: Arc<Notify> = Arc::new(Notify::new());
    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    // Fumador con Papel
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        let notify = Arc::clone(&notify);
        async move {
            fumadores::fumador("Papel", sincronizacion, notify, NUMERO_ITERACIONES).await;
        }
    }));

    // Fumador con Tabaco
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        let notify = Arc::clone(&notify);
        async move {
            fumadores::fumador("Tabaco", sincronizacion, notify, NUMERO_ITERACIONES).await;
        }
    }));

    // Fumador con Fósforos
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        let notify = Arc::clone(&notify);
        async move {
            fumadores::fumador("Fósforos", sincronizacion, notify, NUMERO_ITERACIONES).await;
        }
    }));

    // Agente
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            agente::agente(sincronizacion, NUMERO_ITERACIONES, Duration::from_secs(2)).await;
        }
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}

// Función para ejecutar el ejercicio de los montadores
async fn ejecutar_montadores() {
    let sincronizacion: Arc<semaforo_operarios::SemaforOperarios> = Arc::new(semaforo_operarios::SemaforOperarios::new());
    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    // Operario 1: Ruedas
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            operarios::op1(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    // Operario 2: Cuadro
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            operarios::op2(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    // Operario 3: Manillar
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            operarios::op3(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    // Montador
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            montador::montador(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}
