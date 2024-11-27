use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Represents a reader-writer lock allowing multiple readers or a single writer.
pub struct ReadWriteLock {
    pub readers: Arc<Mutex<u32>>,          // Count of current readers
    pub writer: Arc<Semaphore>,           // Semaphore for writers
    pub read_lock: Arc<Semaphore>,        // Semaphore for reader access control
}

impl ReadWriteLock {
    /// Creates a new `ReadWriteLock`.
    pub fn new() -> Self {
        ReadWriteLock {
            readers: Arc::new(Mutex::new(0)),
            writer: Arc::new(Semaphore::new(1)),  // Only one writer allowed
            read_lock: Arc::new(Semaphore::new(1)),  // Control access for readers
        }
    }
}