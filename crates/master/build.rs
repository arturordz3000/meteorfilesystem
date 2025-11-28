fn main() {
    let out_dir = std::env::var("OUT_DIR");
    println!("cargo:warning=OUT_DIR = {:?}", out_dir);
    println!("cargo:warning=Running build.rs for master!");

    tonic_build::compile_protos("proto/chunk_server_channel.proto").unwrap();
    tonic_build::compile_protos("proto/client_channel.proto").unwrap();
}