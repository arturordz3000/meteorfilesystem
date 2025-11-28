mod pb;
mod chunk_server_channel;

use common::tracing::{info};

use pb::chunk_server_channel::{
    chunk_server_channel_server::{ChunkServerChannelServer},
};
use chunk_server_channel::DefaultChunkServerChannel;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::init_logging();

    let addr = "127.0.0.1:50051".parse()?;

    info!("Server running at {}", addr);

    Server::builder()
        .add_service(ChunkServerChannelServer::new(DefaultChunkServerChannel::default()))
        .serve(addr)
        .await?;

    Ok(())
}
