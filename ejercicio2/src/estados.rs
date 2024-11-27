/// Enumeración que define los posibles estados de un filósofo.
///
/// # Estados
/// - `Pensando`: El filósofo está pensando y no necesita los tenedores.
/// - `TieneHambre`: El filósofo tiene hambre y está esperando los tenedores.
/// - `Comiendo`: El filósofo está comiendo después de haber tomado los tenedores.
#[derive(Clone, Copy, PartialEq)] // Derivaciones necesarias para copiar y comparar valores de estado.
pub enum Estado {
    /// Representa el estado en el que el filósofo está pensando.
    Pensando,
    /// Representa el estado en el que el filósofo tiene hambre y quiere comer.
    TieneHambre,
    /// Representa el estado en el que el filósofo está comiendo.
    Comiendo,
}
