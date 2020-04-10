use crate::fs::{FileSystem, Fs};
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn execute(fs: &FileSystem, file_type: String, file_name: String) -> Result<String, String> {
    if file_type != "blob" {
        return Err("fatal: papyrus cat-file is only implemented for blob types".to_string());
    }

    let current_directory = fs.current_directory();
    let mut full_file_path = PathBuf::from(format!(
        "{}/.papyrus/objects/{}",
        current_directory,
        &file_name[..2]
    ));
    let matching_object_files =
        fs.get_directory_files_starting_with(&full_file_path, &file_name[..].into());

    if matching_object_files.len() == 0 {
        return Err(format!("fatal: Not a valid object name {}", file_name));
    }

    if matching_object_files.len() > 1 {
        return Err(format!(
            "fatal: ambigious argument '{}', there are more than one object with same name",
            file_name
        ));
    }

    full_file_path.push(&matching_object_files[0]);

    let compressed_object_contents = fs.get_file_contents_as_bytes(&full_file_path).unwrap();

    let mut decoder = ZlibDecoder::new(&compressed_object_contents[..]);
    let mut object_contents = String::new();
    decoder.read_to_string(&mut object_contents).unwrap();

    let null_index = object_contents.find("\x00").unwrap() + 1;

    let file_contents = object_contents[null_index..].to_string();

    Ok(file_contents.trim_end().to_string())
}

#[test]
fn test_execute_existing_file() {
    use crate::sub_commands::hash_object;
    let mut fs = FileSystem::access();

    fs.create_file(&"greetings.txt".to_string());
    fs.write_file(&"greetings.txt".to_string(), b"awesome contents yo");

    hash_object::execute(&mut fs, "greetings.txt".into(), true).unwrap();

    assert_eq!(
        execute(
            &fs,
            "blob".into(),
            "5c7f7d83d0da2baceb3789aaf457a699455992fe".into()
        )
        .unwrap(),
        "awesome contents yo"
    );
}

#[test]
fn test_execute_non_existing_file() {
    let fs = FileSystem::access();

    assert_eq!(
        execute(
            &fs,
            "blob".into(),
            "5c7f7d83d0da2baceb3789aaf457a699455992fe".into()
        )
        .unwrap_err(),
        "fatal: Not a valid object name 5c7f7d83d0da2baceb3789aaf457a699455992fe"
    );
}

#[test]
fn test_execute_existing_file_starts_with() {
    use crate::sub_commands::hash_object;
    let mut fs = FileSystem::access();

    fs.create_file(&"greetings.txt".to_string());
    fs.write_file(&"greetings.txt".to_string(), b"awesome contents yo");

    hash_object::execute(&mut fs, "greetings.txt".into(), true).unwrap();

    assert_eq!(
        execute(&fs, "blob".into(), "5c7f7d".into()).unwrap(),
        "awesome contents yo"
    );
}
