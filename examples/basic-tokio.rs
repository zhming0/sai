use sai::{System};
use tokio::signal;

mod gotham_server;
mod db;
mod foo_controller;
mod tide_server;

mod root_registry;
use root_registry::RootRegistry;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system : System<RootRegistry> = System::new();
    println!("System starting up...");
    system.start().await;
    println!("System started.");

    // Waiting for Ctrl-c
    signal::ctrl_c().await?;

    println!("System shutting down...");
    system.stop().await;
    println!("System shutted down.");
    Ok(())
}
