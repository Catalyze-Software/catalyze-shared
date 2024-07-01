use std::{env, fs::write, path::PathBuf};

pub fn save_candid_file(path: &str, contents: String) {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().parent().unwrap().join("candid");
    write(dir.join(path), contents).expect("Write failed.");
}
