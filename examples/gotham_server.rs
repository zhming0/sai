use sai::{Component, Injected, async_trait};
use futures::prelude::*;
use futures::channel::oneshot::{Sender, Receiver};
use gotham::state::State;
use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DrawRoutes};
use gotham::router::builder::DefineSingleRoute;

#[derive(Debug)]
enum ServerCommand {
    Stop,
    Stopped // Technically this is a event, but I am lazy
}

fn index(state: State) -> (State, &'static str) {
    //std::thread::sleep(std::time::Duration::from_secs(10));
    //println!("---handle index---");
    (state, "hello")
}

#[derive(Component)]
#[lifecycle]
pub struct GothamServer {
    stop_command_sender: Option<Sender<ServerCommand>>,
    stop_ack_receiver: Option<Receiver<ServerCommand>>
}

#[async_trait]
impl sai::ComponentLifecycle for GothamServer {
    async fn start (&mut self) {
        println!("Starting Gotham Server...");
        let ( sender, receiver ) = futures::channel::oneshot::channel::<ServerCommand>();
        let ( ack_sender, ack_receiver ) = futures::channel::oneshot::channel::<ServerCommand>();

        self.stop_command_sender = Some(sender);
        self.stop_ack_receiver = Some(ack_receiver);

        // Ensure the server runs in the background
        tokio::spawn(async {

            let router = {
                build_simple_router(|route| {
                    route
                        .get("hello")
                        .to(index);
                    })
            };

            let server_fut = gotham::init_server("0.0.0.0:9002", router);

            let server_handle_fut = async {
                // Receiver's ownership is transfered here
                receiver.await.unwrap();
            };

            /*
             * When shutting down server_handle_fut will return first because it's
             * activated by stop_command_sender
             *
             * Then this `select` will drop the `server` and make it shutting down gracefully
             */
            future::select(
                server_fut.boxed(),
                server_handle_fut.boxed()
            ).await;

            println!("Server gracefully shutted down...");
            // Technically, the server hasn't being shutted down 100%
            // Because it's up to runtime to drop those spawned tasks

            // Acknowledge that the server has shutted down
            ack_sender.send(ServerCommand::Stopped).unwrap();
        });
    }

    async fn stop (&mut self) {
        println!("Shutting down web server...");
        // It's important to `take` here
        let sender = self.stop_command_sender.take().unwrap();
        sender.send(ServerCommand::Stop).unwrap();

        // Ensure this future returns only when the server has gracefully shutted down
        let ack_receiver = self.stop_ack_receiver.take().unwrap();
        ack_receiver.await.unwrap();
    }
}
