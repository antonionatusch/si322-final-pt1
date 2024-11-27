mod bear_bees;
mod menu;

#[tokio::main]
async fn main() {
    menu::run_menu().await;
}
