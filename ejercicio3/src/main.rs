/// Módulos que encapsulan la lógica de los fumadores, montadores y su sincronización.
mod fumadores; // Módulo para la lógica de los fumadores.
mod semaforo_fumadores; // Módulo para la sincronización entre fumadores y el agente.
mod agente; // Módulo para manejar al agente.

mod operarios; // Módulo para la lógica de los operarios.
mod montador; // Módulo para la lógica del montador.
mod semaforo_operarios; // Módulo para la sincronización entre operarios y montador.

use std::sync::Arc;
use tokio::sync::Notify;
use std::io::{self, Write};
use std::time::Duration;

/// Variable global que define el número máximo de iteraciones para cada ejercicio.
/// Controla cuántas veces los fumadores y los montadores ejecutan sus tareas.
const NUMERO_ITERACIONES: usize = 5;

#[tokio::main]
/// Función principal que maneja el menú interactivo para seleccionar entre
/// los ejercicios de fumadores o montadores.
///
/// Continúa ejecutándose en un bucle hasta que el usuario seleccione "Salir".
async fn main() {
    loop {
        // Despliega el menú principal
        println!("--- Menú Principal ---");
        println!("1. Ejecutar ejercicio de los fumadores de cigarrillos");
        println!("2. Ejecutar ejercicio de los montadores de bicicletas");
        println!("3. Salir");
        print!("Seleccione una opción: ");
        io::stdout().flush().unwrap(); // Asegura que el texto se imprima antes de leer la entrada del usuario.

        // Lee la opción del usuario desde la consola
        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();
        let opcion = opcion.trim();

        // Ejecuta la opción seleccionada por el usuario
        match opcion {
            "1" => {
                println!("Iniciando el ejercicio de los fumadores de cigarrillos...");
                ejecutar_fumadores().await; // Llama al ejercicio de fumadores
            }
            "2" => {
                println!("Iniciando el ejercicio de los montadores de bicicletas...");
                ejecutar_montadores().await; // Llama al ejercicio de montadores
            }
            "3" => {
                println!("Saliendo del programa...");
                break; // Finaliza el bucle y el programa.
            }
            _ => {
                println!("Opción inválida. Intente nuevamente."); // Maneja opciones no válidas.
            }
        }
    }
}

/// Función que inicializa y ejecuta el ejercicio de los fumadores.
///
/// Esta función configura las tareas asincrónicas para los fumadores y el agente,
/// y las coordina utilizando semáforos y notificaciones.
async fn ejecutar_fumadores() {
    // Configura la sincronización entre fumadores y el agente.
    let sincronizacion: Arc<semaforo_fumadores::SemphoreSmoker> = Arc::new(semaforo_fumadores::SemphoreSmoker::new());
    let notify: Arc<Notify> = Arc::new(Notify::new()); // Notificación compartida entre tareas.
    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![]; // Vector para almacenar las tareas.

    // Tarea para el fumador con papel
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        let notify = Arc::clone(&notify);
        async move {
            fumadores::fumador("Papel", sincronizacion, notify, NUMERO_ITERACIONES).await;
        }
    }));

    // Tarea para el fumador con tabaco
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        let notify = Arc::clone(&notify);
        async move {
            fumadores::fumador("Tabaco", sincronizacion, notify, NUMERO_ITERACIONES).await;
        }
    }));

    // Tarea para el fumador con fósforos
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        let notify = Arc::clone(&notify);
        async move {
            fumadores::fumador("Fósforos", sincronizacion, notify, NUMERO_ITERACIONES).await;
        }
    }));

    // Tarea para el agente
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            agente::agente(sincronizacion, NUMERO_ITERACIONES, Duration::from_secs(2)).await;
        }
    }));

    // Espera a que todas las tareas terminen.
    for handle in handles {
        handle.await.unwrap();
    }
}

/// Función que inicializa y ejecuta el ejercicio de los montadores.
///
/// Esta función configura las tareas asincrónicas para los operarios y el montador,
/// y las coordina utilizando semáforos.
async fn ejecutar_montadores() {
    // Configura la sincronización entre operarios y montador.
    let sincronizacion: Arc<semaforo_operarios::SemaforOperarios> = Arc::new(semaforo_operarios::SemaforOperarios::new());
    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![]; // Vector para almacenar las tareas.

    // Tarea para el operario 1 (ruedas)
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            operarios::op1(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    // Tarea para el operario 2 (cuadro)
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            operarios::op2(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    // Tarea para el operario 3 (manillar)
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            operarios::op3(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    // Tarea para el montador
    handles.push(tokio::spawn({
        let sincronizacion = Arc::clone(&sincronizacion);
        async move {
            montador::montador(sincronizacion, NUMERO_ITERACIONES).await;
        }
    }));

    // Espera a que todas las tareas terminen.
    for handle in handles {
        handle.await.unwrap();
    }
}
