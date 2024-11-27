mod mesa;
mod filosofos;
mod estados;
mod semaforo_filosofos;

use crate::mesa::Mesa;
use crate::filosofos::filosofo;
use std::sync::Arc;
use tokio;
#[tokio::main]
async fn main() {
    const N: usize = 5;
    let max_comiendo = 2; // Máximo número de filósofos que pueden comer simultáneamente
    let mesa = Arc::new(Mesa::new(max_comiendo));
    let mut handles = vec![];

    for i in 0..N {
        let mesa_clone = Arc::clone(&mesa);
        handles.push(tokio::spawn(async move {
            filosofo(i, mesa_clone).await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
