pub mod chunkserver {
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::{Instant};

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ChunkServerHealthInformation {
        pub free_space_bytes: i64,
        pub avg_cpu_usage: i32,
        pub avg_memory_usage: i32,
    }
    
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ChunkServerMetadata {
        pub server_id: String,
        pub ip_address: String,
        pub last_heartbeat: Instant,
        pub health_information: Option<ChunkServerHealthInformation>,
    }

    pub type ChunkServersRegistry = Mutex<HashMap<String, ChunkServerMetadata>>;
}
