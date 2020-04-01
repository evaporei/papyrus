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

pub fn execute(fs: &mut FileSystem, file_name: PathBuf, write: bool) -> Result<String, String> {
    let contents = fs.get_file_contents(&file_name)?;

    let sha1 = create_sha1(&contents);

    if write {
        let object_folder = &sha1[..2];
        let object_file = &sha1[2..];

        let mut absolute_folder_path = PathBuf::from(fs.current_directory());
        absolute_folder_path.push(".papyrus");
        absolute_folder_path.push("objects");
        absolute_folder_path.push(object_folder);

        fs.create_directory(&absolute_folder_path);

        let mut absolute_file_path = absolute_folder_path.clone();
        absolute_file_path.push(object_file);
        absolute_file_path.set_file_name(object_file);

        fs.create_file(&absolute_file_path);
    }

    Ok(sha1)
}

#[test]
fn test_execute_existing_file() {
    let mut fs = FileSystem::access();

    assert_eq!(
        execute(&mut fs, "example.txt".into(), false).unwrap(),
        "ba092ead72a64a26d7877e66d4b97640f8cd9301"
    );
}

#[test]
fn test_execute_writing_existing_file() {
    let mut fs = FileSystem::access();

    assert_eq!(
        execute(&mut fs, "example.txt".into(), true).unwrap(),
        "ba092ead72a64a26d7877e66d4b97640f8cd9301"
    );

    assert!(fs.path_exists(&format!("{}/.papyrus/objects/ba", fs.current_directory())));
    assert!(fs.path_exists(&format!(
        "{}/.papyrus/objects/ba/092ead72a64a26d7877e66d4b97640f8cd9301",
        fs.current_directory()
    )));
}

#[test]
fn test_execute_non_existing_file() {
    let mut fs = FileSystem::access();

    assert_eq!(
        execute(&mut fs, "non_existing_file.txt".into(), false).unwrap_err(),
        "fatal: Cannot open '\"non_existing_file.txt\"': No such file or directory (os error 2)"
    );
}
