mod buffer;
mod consumer;
mod producer;
mod semaphore;

use buffer::Buffer;
use consumer::Consumer;
use producer::Producer;
use semaphore::CustomSemaphore;
use tokio::task;

#[tokio::main]
async fn main() {
    const BUFFER_SIZE: usize = 100;

    let buffer = Buffer::new(BUFFER_SIZE);
    let empty_slots = CustomSemaphore::new(BUFFER_SIZE);
    let full_slots = CustomSemaphore::new(0);

    let producer = Producer::new(buffer.clone(), empty_slots.clone(), full_slots.clone());
    let consumer = Consumer::new(buffer.clone(), empty_slots.clone(), full_slots.clone());

    let producer_handle = task::spawn(async move {
        producer.produce().await;
    });

    let consumer_handle = task::spawn(async move {
        consumer.consume().await;
    });

    let _ = tokio::join!(producer_handle, consumer_handle);
}
