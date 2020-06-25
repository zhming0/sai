use shine::{Component, Injected, async_trait};
use futures::prelude::*;
use futures::channel::oneshot::{Sender, Receiver};

use super::foo_controller::FooController;

#[derive(Debug)]
enum ServerCommand {
    Stop,
    Stopped // Technically this is a event, but I am lazy
}

#[derive(Component)]
#[lifecycle]
pub struct TideServer {
    #[injected]
    foo_controller: Injected<FooController>,

    stop_command_sender: Option<Sender<ServerCommand>>,
    stop_ack_receiver: Option<Receiver<ServerCommand>>
}

#[async_trait]
impl shine::ComponentLifecycle for TideServer {
    async fn start (&mut self) {
        println!("Starting TideServer...");
        let ( sender, receiver ) = futures::channel::oneshot::channel::<ServerCommand>();
        let ( ack_sender, ack_receiver ) = futures::channel::oneshot::channel::<ServerCommand>();

        self.stop_command_sender = Some(sender);
        self.stop_ack_receiver = Some(ack_receiver);

        struct State {
            foo_controller: Injected<FooController>
        };

        let state = State {
            foo_controller: self.foo_controller.clone()
        };


        // Ensure the server runs in the background
        tokio::spawn(async {

            let server_handle_fut = async {
                // Receiver's ownership is transfered here
                receiver.await.unwrap();
            };

            let mut app = tide::with_state(state);
            app.at("/").get(|_| async { Ok("Hello, world!") });
            app.at("/foo").get(|req: tide::Request<State>| async move {
                let f = &req.state().foo_controller;
                return f.extract().index();
            });
            let server_fut = app.listen("0.0.0.0:9003");

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
