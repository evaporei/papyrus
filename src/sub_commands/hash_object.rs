use crate::fs::{FileSystem, Fs};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::path::PathBuf;

fn create_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    hasher.result_str()
}

#[test]
fn test_create_sha1() {
    assert_eq!(
        create_sha1("contents\nanother line"),
        "ba092ead72a64a26d7877e66d4b97640f8cd9301"
    );
}

pub fn execute(file_name: PathBuf) -> Result<String, String> {
    let contents = FileSystem::access().get_file_contents(&file_name)?;

    Ok(create_sha1(&contents))
}

#[test]
fn test_execute() {
    assert_eq!(
        execute("example.txt".into()).unwrap(),
        "ba092ead72a64a26d7877e66d4b97640f8cd9301"
    );
    assert_eq!(
        execute("non_existing_file.txt".into()).unwrap_err(),
        "fatal: Cannot open '\"non_existing_file.txt\"': No such file or directory (os error 2)"
    );
}
