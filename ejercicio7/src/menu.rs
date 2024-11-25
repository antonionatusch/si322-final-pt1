use crate::server;
use crate::printer;
use crate::user;

pub async fn problema_17() {
    server::start_connection().await;
}

pub fn problema_18_a() {
    printer::use_printer();
}

pub fn problema_18_b() {
    user::use_printer_with_priority();
}