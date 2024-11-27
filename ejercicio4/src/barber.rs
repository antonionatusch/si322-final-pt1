use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Represents a barber shop with a limited number of chairs and customers.
pub struct BarberShop {
    pub semaphore: Arc<Semaphore>,                // Semaphore to manage available chairs
    pub max_chairs: u32,                          // Maximum number of chairs
    pub customers_waiting: Arc<Mutex<u32>>,       // Number of customers waiting
    pub remaining_customers: Arc<Mutex<u32>>,     // Total remaining customers
}

impl BarberShop {
    /// Creates a new `BarberShop` with the specified number of chairs and total customers.
    ///
    /// # Arguments
    ///
    /// * `max_chairs` - The maximum number of chairs available in the barber shop.
    /// * `total_customers` - The total number of customers expected.
    pub fn new(max_chairs: u32, total_customers: u32) -> Self {
        BarberShop {
            semaphore: Arc::new(Semaphore::new(max_chairs as usize)),
            max_chairs,
            customers_waiting: Arc::new(Mutex::new(0)),
            remaining_customers: Arc::new(Mutex::new(total_customers)),
        }
    }
}