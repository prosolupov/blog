fn main() {
    println!("cargo:rerun-if-changed=proto/blog.proto");

    let protoc = protoc_bin_vendored::protoc_bin_path().expect("failed to find bundled protoc");
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(&["proto/blog.proto"], &["proto"])
        .expect("failed to compile protos");
}
