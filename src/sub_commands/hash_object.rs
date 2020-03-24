use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::read_to_string;
use std::path::PathBuf;

fn get_file_contents(file_name: &PathBuf) -> Result<String, String> {
    read_to_string(&file_name)
        .map_err(|err| format!("fatal: Cannot open '{:?}': {}", file_name, err))
}

fn create_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn execute(file_name: PathBuf) -> Result<String, String> {
    let contents = get_file_contents(&file_name)?;

    Ok(create_sha1(&contents))
}

#[test]
fn test_create_sha1() {
    assert_eq!(create_sha1("abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
}
