mod model;

use std::{sync::mpsc::{Sender, Receiver, channel}, thread::JoinHandle, time::Duration};

use common::tracing::{info, error, debug};
use common::master_channel::{master_channel_client::MasterChannelClient};
use model::ThreadEvents;

fn main() {
    common::init_logging();

    let (heartbeat_tx, heartbeat_rx) = channel::<ThreadEvents>();
    let heartbeat_thread = start_heartbeat_thread(heartbeat_rx);

    // TODO: start RPC chunkserver server

    shutdown_and_join_heartbeat_thread(&heartbeat_tx, heartbeat_thread);
}

fn start_heartbeat_thread(heartbeat_rx: Receiver<ThreadEvents>) -> JoinHandle<()>  {
    info!("Starting heartbeat thread...");

    return std::thread::spawn(move || {
        loop {
            // TODO: send heartbeat event to master node

            let message = heartbeat_rx.try_recv();

            match message {
                Ok(ThreadEvents::Shutdown) => {
                    info!("Shutdown received in heartbeat thread. Stopping thread...");
                    break;
                }

                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    debug!("No messages in heartbeat thread. Thread will sleep for 10 seconds...");
                    std::thread::sleep(Duration::from_secs(10));
                }
    
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    error!("Heartbeat channel disconnected. Stopping heartbeat thread...");
                    panic!()
                }
            }
        }
    }); 
}

fn shutdown_and_join_heartbeat_thread(heartbeat_tx: &Sender<ThreadEvents>, heartbeat_thread: JoinHandle<()>) {
    let heartbeat_shutdown_message_result = heartbeat_tx.send(ThreadEvents::Shutdown);

    match heartbeat_shutdown_message_result {
        Ok(_) => { info!("Shutdown sent to heartbeat thread"); },
        Err(_) => { error!("Couldn't send shutdown to heartbeat thread") },
    }

    heartbeat_thread.join().unwrap_or_else(|e| {
        error!("Heartbeat thread panicked with the following error: {:?}", e);
    });
}
