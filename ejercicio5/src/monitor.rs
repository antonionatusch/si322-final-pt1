use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ReaderWriterMonitor {
    reader_count: Mutex<usize>,    // Número de lectores activos
    writer_active: Mutex<bool>,   // Si un escritor está activo
}

impl ReaderWriterMonitor {
    /// Crea una nueva instancia de `ReaderWriterMonitor`.
    ///
    /// Inicializa los contadores y banderas necesarias para la sincronización.
    /// - `reader_count` comienza en 0 (sin lectores activos).
    /// - `writer_active` comienza en `false` (ningún escritor está activo).
    pub fn new() -> Self {
        Self {
            reader_count: Mutex::new(0),
            writer_active: Mutex::new(false),
        }
    }

    /// Permite que un lector inicie la lectura.
    ///
    /// Este método asegura que los lectores solo accedan al recurso compartido si no hay escritores activos.
    /// Si un escritor está activo, el lector espera y reintenta después de un breve tiempo.
    ///
    /// - Incrementa el contador de lectores (`reader_count`) si no hay escritores activos.
    /// - Utiliza espera activa (`thread::sleep`) para manejar la concurrencia.
    pub fn start_read(&self) {
        loop {
            let writer_active = self.writer_active.lock().unwrap();
            if !*writer_active {
                // Si no hay escritores activos, incrementar contador de lectores y salir del bucle
                let mut reader_count = self.reader_count.lock().unwrap();
                *reader_count += 1;
                return;
            }
            // Esperar brevemente antes de reintentar
            drop(writer_active);
            thread::sleep(Duration::from_millis(50));
        }
    }

    /// Permite que un lector termine la lectura.
    ///
    /// Este método decrementa el contador de lectores activos (`reader_count`). No notifica directamente
    /// a otros hilos, ya que el control de espera está implementado en los métodos de escritores.
    pub fn end_read(&self) {
        let mut reader_count = self.reader_count.lock().unwrap();
        *reader_count -= 1;
    }

    /// Permite que un escritor inicie la escritura.
    ///
    /// Este método asegura que un escritor solo acceda al recurso compartido si:
    /// - No hay otros escritores activos (`writer_active` es `false`).
    /// - No hay lectores activos (`reader_count` es `0`).
    ///
    /// Si cualquiera de estas condiciones no se cumple, el escritor espera y reintenta después de un breve tiempo.
    ///
    /// - Marca el inicio de la escritura configurando `writer_active` en `true`.
    /// - Utiliza espera activa (`thread::sleep`) para manejar la concurrencia.
    pub fn start_write(&self) {
        loop {
            let mut writer_active = self.writer_active.lock().unwrap();
            let reader_count = self.reader_count.lock().unwrap();

            if !*writer_active && *reader_count == 0 {
                // Si no hay escritores ni lectores activos, marcar que un escritor está escribiendo
                *writer_active = true;
                return;
            }
            // Esperar brevemente antes de reintentar
            drop(writer_active);
            drop(reader_count);
            thread::sleep(Duration::from_millis(50));
        }
    }

    /// Permite que un escritor termine la escritura.
    ///
    /// Este método libera el acceso exclusivo del escritor al recurso compartido:
    /// - Configura `writer_active` en `false` para permitir el acceso a lectores o escritores.
    pub fn end_write(&self) {
        let mut writer_active = self.writer_active.lock().unwrap();
        *writer_active = false;
    }
}
