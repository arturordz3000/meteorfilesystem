fn main() {
    let out_dir = std::env::var("OUT_DIR");
    println!("cargo:warning=OUT_DIR = {:?}", out_dir);
    println!("cargo:warning=Running build.rs for master!");

    tonic_build::compile_protos("proto/chunkserver_channel.proto").unwrap();
    tonic_build::compile_protos("proto/master_channel.proto").unwrap();
}