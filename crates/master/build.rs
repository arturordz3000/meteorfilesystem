fn main() {
    tonic_build::compile_protos("proto/chunk_server_channel.proto").unwrap();
    tonic_build::compile_protos("proto/client_channel.proto").unwrap();
}