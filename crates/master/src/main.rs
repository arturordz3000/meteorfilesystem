mod model;
mod master_channel;

use common::tracing::{info};

use common::master_channel::{
    master_channel_server::{MasterChannelServer},
};
use master_channel::DefaultMasterChannel;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::init_logging();

    let address = "127.0.0.1:50051".parse()?;

    info!("Server running at {}", address);

    Server::builder()
        .add_service(MasterChannelServer::new(DefaultMasterChannel::default()))
        .serve(address)
        .await?;

    Ok(())
}
