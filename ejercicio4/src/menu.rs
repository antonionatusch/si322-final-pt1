use crate::read_write_lock_impl::run_readers_writers;
use crate::barber_impl::run_barber_shop;

pub async fn problem_12() {
    run_readers_writers().await;
}

pub async fn problem_13() {
    run_barber_shop().await;
}