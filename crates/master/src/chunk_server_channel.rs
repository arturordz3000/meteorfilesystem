use std::collections::hash_map::Entry;
use std::time::Instant;
use std::{collections::HashMap, sync::Mutex};

use common::tracing::{debug, error};
use tonic::{Request, Response, Status};
use once_cell::sync::Lazy;
use crate::model::chunkserver::ChunkServerHealthInformation;
use crate::pb::chunk_server_channel::{
    HeartbeatRequest, HeartbeatResponse, RequestedAction, chunk_server_channel_server::ChunkServerChannel
};
use crate::model::{
    chunkserver::{ChunkServerMetadata, ChunkServersRegistry},
};

pub static CHUNKSERVER_REGISTRY: Lazy<ChunkServersRegistry> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Default)]
pub struct DefaultChunkServerChannel;

#[tonic::async_trait]
impl ChunkServerChannel for DefaultChunkServerChannel {
    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        let mut registry = CHUNKSERVER_REGISTRY.lock().unwrap_or_else(|e| {
            error!("CHUNKSERVER_REGISTRY mutex poisoned with error: {}", e);
            panic!();
        });
        
        let heartbeat = request.get_ref();

        debug!("Heartbeat from {}", heartbeat.server_id);

        let mut reply = HeartbeatResponse {
            requested_action: RequestedAction::None.into()
        };

        match registry.entry(heartbeat.server_id.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(ChunkServerMetadata { 
                    server_id: heartbeat.server_id.clone(), 
                    last_heartbeat: Instant::now(),
                    ip_address: heartbeat.ip_address.clone(),
                    health_information: heartbeat.health_information.clone().map(|h| {
                        ChunkServerHealthInformation { 
                            free_space_bytes: h.free_space_bytes, 
                            avg_cpu_usage: h.avg_cpu_usage, 
                            avg_memory_usage: h.avg_memory_usage 
                        }
                    })
                });

                reply.requested_action = RequestedAction::ChunkListReport.into();
            }

            Entry::Occupied(mut entry) => {
                let metadata = entry.get_mut();

                metadata.last_heartbeat = Instant::now();
                metadata.ip_address = metadata.ip_address.clone();
                metadata.health_information = heartbeat.health_information.clone().map(|h| {
                    ChunkServerHealthInformation { 
                        free_space_bytes: h.free_space_bytes, 
                        avg_cpu_usage: h.avg_cpu_usage, 
                        avg_memory_usage: h.avg_memory_usage 
                    }
                });
            }
        }

        Ok(Response::new(reply))
    }
}
