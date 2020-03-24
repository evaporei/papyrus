use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::read_to_string;
use std::path::PathBuf;

fn create_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn execute(file_name: PathBuf) {
    let contents = read_to_string(file_name).unwrap();

    println!("{}", create_sha1(&contents));
}
