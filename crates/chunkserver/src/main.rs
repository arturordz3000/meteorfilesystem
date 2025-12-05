mod model;

use std::{sync::mpsc::{Receiver, channel}, thread::JoinHandle, time::Duration};

use common::tracing::{info, error};
use model::ThreadEvents;

fn main() {
    common::init_logging();

    let (_heartbeat_tx, heartbeat_rx) = channel::<ThreadEvents>();
    let heartbeat_thread = start_heartbeat_thread(heartbeat_rx);

    // TODO: start RPC chunkserver server

    heartbeat_thread.join().unwrap_or_else(|e| {
        error!("Heartbeat thread panicked with the following error: {:?}", e);
    });
}

fn start_heartbeat_thread(heartbeat_rx: Receiver<ThreadEvents>) -> JoinHandle<()>  {
    info!("Starting heartbeat thread...");

    return std::thread::spawn(move || {
        loop {
            // TODO: send heartbeat event to master node

            let message = heartbeat_rx.try_recv();

            match message {
                Ok(event) => {
                    match event {
                        ThreadEvents::Shutdown => {
                            info!("Shutdown event received in heartbeat thread. Stopping thread...");
                            break;
                        }

                        _ => {
                            std::thread::sleep(Duration::from_secs(10));
                        }
                    }
                }

                Err(error) => {
                    error!("Unable to receive messages in heartbeat channel: {:?}", error);
                }
            }
        }
    }); 
}
