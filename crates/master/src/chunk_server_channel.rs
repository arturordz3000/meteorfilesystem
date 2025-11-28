use tonic::{Request, Response, Status};

use crate::pb::chunk_server_channel::{
    chunk_server_channel_server::{ChunkServerChannel},
    HeartbeatRequest, HeartbeatResponse, RequestedAction
};

#[derive(Default)]
pub struct DefaultChunkServerChannel;

#[tonic::async_trait]
impl ChunkServerChannel for DefaultChunkServerChannel {
    async fn heartbeat(
        &self,
        _: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        let reply = HeartbeatResponse {
            requested_action: RequestedAction::None.into()
        };

        Ok(Response::new(reply))
    }
}
