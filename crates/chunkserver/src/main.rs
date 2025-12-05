mod model;

use std::{sync::mpsc::{Sender, Receiver, channel}, time::Duration};

use tokio::task::JoinHandle;

use common::{master_channel::HeartbeatRequest, tracing::{debug, error, info}};
use common::master_channel::{master_channel_client::MasterChannelClient};
use model::ThreadEvents;

#[tokio::main]
async fn main() {
    common::init_logging();

    let (heartbeat_tx, heartbeat_rx) = channel::<ThreadEvents>();
    let heartbeat_thread = start_heartbeat_thread(heartbeat_rx);

    // TODO: start RPC chunkserver server

    shutdown_and_join_heartbeat_thread(&heartbeat_tx, heartbeat_thread).await;
}

fn start_heartbeat_thread(heartbeat_rx: Receiver<ThreadEvents>) -> JoinHandle<()>  {
    info!("Starting heartbeat thread...");

    return tokio::spawn(async move {
        loop {
            let mut master_client = MasterChannelClient::connect("http://127.0.0.1:50051").await.unwrap_or_else(|error| {
                error!("Could not connect to master from heartbeat thread: {:?}", error);
                panic!();
            });

            if let Err(error) = master_client.heartbeat(HeartbeatRequest {
                server_id: "Server1".to_string(),
                ip_address: "127.0.0.1".to_string(),
                health_information: None
            }).await {
                error!("Error sending heartbeat to master: {:?}", error);
            }

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

async fn shutdown_and_join_heartbeat_thread(heartbeat_tx: &Sender<ThreadEvents>, heartbeat_thread: JoinHandle<()>) {
    // TODO: uncomment the following code after we add the chunkserver rpc channel

    // let heartbeat_shutdown_message_result = heartbeat_tx.send(ThreadEvents::Shutdown);

    // match heartbeat_shutdown_message_result {
    //     Ok(_) => { info!("Shutdown sent to heartbeat thread"); },
    //     Err(_) => { error!("Couldn't send shutdown to heartbeat thread") },
    // }

    heartbeat_thread.await.unwrap_or_else(|e| {
        error!("Heartbeat thread panicked with the following error: {:?}", e);
    });
}
