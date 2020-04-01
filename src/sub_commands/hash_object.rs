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

pub fn execute(fs: &mut FileSystem, file_name: PathBuf) -> Result<String, String> {
    let contents = fs.get_file_contents(&file_name)?;

    Ok(create_sha1(&contents))
}

#[test]
fn test_execute_existing_file() {
    let mut fs = FileSystem::access();

    assert_eq!(
        execute(&mut fs, "example.txt".into()).unwrap(),
        "ba092ead72a64a26d7877e66d4b97640f8cd9301"
    );
}

#[test]
fn test_execute_non_existing_file() {
    let mut fs = FileSystem::access();

    assert_eq!(
        execute(&mut fs, "non_existing_file.txt".into()).unwrap_err(),
        "fatal: Cannot open '\"non_existing_file.txt\"': No such file or directory (os error 2)"
    );
}
