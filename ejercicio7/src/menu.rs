use crate::server;
use crate::printer;
use crate::user;

/// Funcion del ejercicio 17
/// 
/// Para este problema un servidor puede permitir
/// N conexiones simultaneas
pub async fn problema_17() {
    server::start_connection().await;
}

/// Funcion del ejercicio 18 A
/// 
/// Para este problema una impresaroa permite N usuarios
pub fn problema_18_a() {
    printer::use_printer();
}

/// Funcion del ejercicio 18 B
/// 
/// Para este problema ademas hay prioridad por usuario
pub fn problema_18_b() {
    user::use_printer_with_priority();
}