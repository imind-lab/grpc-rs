use std::{path::PathBuf, env};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("micro_descriptor.bin"))
        .compile(&["../proto/micro.proto"], &["../proto"])
        .unwrap();
}