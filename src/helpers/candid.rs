use std::{fs::OpenOptions, io::Write};

pub fn save_candid_file(path: &str, contents: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Unable to open file for writing candid file, path: {path}"));

    file.write_all(contents.as_bytes())
        .expect("Unable to write candid file");
    file.flush().expect("Unable to flush candid file");
}
