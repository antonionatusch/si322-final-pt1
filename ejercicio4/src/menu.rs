use crate::barber_impl::run_barber_shop;
use crate::read_write_lock_impl::run_readers_writers;

pub async fn problem_12() {
    run_barber_shop().await;
}

pub async fn problem_13() {
    run_readers_writers().await;
}
