use anyhow::Result as AnyResult;
use bastion::Bastion as BASTION;
use std::thread;
use tracing::Level;

mod config;

mod adapters_primary;
mod adapters_secondary;
mod domain;

#[tokio::main]
async fn main() -> AnyResult<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    BASTION::init();

    // Adapters Secondary
    adapters_secondary::fight::Random::init()?;
    adapters_secondary::chat::API::init()?;

    // Context
    let fight_context =
        domain::fight::Context::new("fight_adapter", "chat_adapter", String::from("Rabs"));

    BASTION::start();

    // Adapters Primary
    let websocket_thread =
        thread::spawn(|| adapters_primary::websocket::start_websocket(fight_context));

    let _ = websocket_thread.join();

    BASTION::stop();
    BASTION::block_until_stopped();

    Ok(())
}
