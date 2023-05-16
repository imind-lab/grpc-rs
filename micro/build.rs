use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=../proto/micro.proto");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("micro_descriptor.bin"))
        .compile(&["../proto/micro.proto"], &["../proto"])
        .unwrap();
}
