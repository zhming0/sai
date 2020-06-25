use shine::{Component, component_registry, System, Injected};
use std::any::TypeId;
use tokio::signal;

mod gotham_server;
use gotham_server::GothamServer;

mod db;
use db::Db;

mod foo_controller;

mod tide_server;
use tide_server::TideServer;

component_registry!(RootRegistry, [
    GothamServer,
    Db,
    foo_controller::FooController,
    TideServer
]);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system : System<RootRegistry> = System::new(
        TypeId::of::<Injected<TideServer>>()
    );
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
