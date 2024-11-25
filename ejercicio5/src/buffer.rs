pub struct Buffer {
    pub data: Vec<i32>,
    pub capacity: usize,
}

impl Buffer {
    /// Crea un nuevo buffer con una capacidad definida.
    /// 
    /// # Parámetros
    /// - `capacity`: capacidad máxima del buffer.
    /// 
    /// # Retorno
    /// Retorna una instancia de la estructura `Buffer` inicializada con un vector vacío y la capacidad especificada.
    pub fn new(capacity: usize) -> Self {
        Buffer {
            data: Vec::new(),
            capacity,
        }
    }

    /// Verifica si el buffer está lleno.
    /// 
    /// # Retorno
    /// Retorna `true` si la cantidad de elementos en el buffer es igual a su capacidad máxima, de lo contrario retorna `false`.
    pub fn is_full(&self) -> bool {
        self.data.len() == self.capacity
    }

    /// Verifica si el buffer está vacío.
    /// 
    /// # Retorno
    /// Retorna `true` si no hay elementos en el buffer, de lo contrario retorna `false`.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}