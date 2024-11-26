use crate::bear_bees::{bear_task, bee_task, HoneyJar};
use std::sync::Arc;
use tokio::task;

/// Muestra el menú y permite configurar la simulación.
pub async fn run_menu() {
    println!("Bienvenido a la simulación del oso y las abejas.");
    println!("Ingrese el número de abejas:");
    let mut bees_input = String::new();
    std::io::stdin()
        .read_line(&mut bees_input)
        .expect("Error al leer la entrada.");
    let num_bees: usize = bees_input
        .trim()
        .parse()
        .expect("Debe ingresar un número válido.");

    println!("Ingrese la capacidad del tarro de miel:");
    let mut capacity_input = String::new();
    std::io::stdin()
        .read_line(&mut capacity_input)
        .expect("Error al leer la entrada.");
    let capacity: usize = capacity_input
        .trim()
        .parse()
        .expect("Debe ingresar un número válido.");

    let honey_jar = HoneyJar::new(capacity);

    // Inicia la tarea del oso.
    let bear = task::spawn(bear_task(honey_jar.clone()));

    // Inicia las tareas de las abejas.
    let mut bee_tasks = Vec::new();
    for bee_id in 1..=num_bees {
        let jar_clone = honey_jar.clone();
        bee_tasks.push(task::spawn(bee_task(jar_clone, bee_id)));
    }

    println!("Simulación en ejecución. Presione Ctrl+C para salir.");
    use tokio::task;

    let _ = tokio::join!(bear, async {
        for task in bee_tasks {
            task.await.unwrap();
        }
    });
}
