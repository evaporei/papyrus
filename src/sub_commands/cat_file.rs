use crate::fs::{FileSystem, Fs};
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use std::path::PathBuf;

const POSSIBLE_FIRST_PARAMETER: [&str; 3] = ["-t", "blob", "tree"];

pub fn execute(
    fs: &FileSystem,
    file_type_or_type_flag: String,
    file_name: String,
) -> Result<String, String> {
    if !POSSIBLE_FIRST_PARAMETER
        .iter()
        .any(|p| p == &file_type_or_type_flag)
    {
        return Err(format!(
            "fatal: papyrus cat-file first parameter can only receive one of ({})",
            POSSIBLE_FIRST_PARAMETER.join(", ")
        ));
    }

    let current_directory = fs.current_directory();
    let mut full_file_path = PathBuf::from(format!(
        "{}/.papyrus/objects/{}",
        current_directory,
        &file_name[..2]
    ));
    let matching_object_files =
        fs.get_directory_files_starting_with(&full_file_path, &file_name[..].into());

    if matching_object_files.is_empty() {
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

    match &file_type_or_type_flag[..] {
        "blob" | "tree" => Ok(get_object_data(&object_contents)),
        "-t" => Ok(get_file_type(&object_contents)),
        _ => unreachable!(),
    }
}

fn get_object_data(object_contents: &str) -> String {
    let null_index = object_contents.find('\x00').unwrap() + 1;

    let object_data = object_contents[null_index..].to_string();

    object_data.trim_end().to_string()
}

fn get_file_type(object_contents: &str) -> String {
    let space_index = object_contents.find(' ').unwrap();

    let file_type = object_contents[..space_index].to_string();

    file_type.to_string()
}

#[test]
fn test_execute_existing_file_contents() {
    use crate::sub_commands::hash_object;
    let mut fs = FileSystem::access();

    let example_content = b"awesome contents yo";

    hash_object::execute(&mut fs, example_content, "blob".into(), true).unwrap();

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

    let example_content = b"awesome contents yo";

    hash_object::execute(&mut fs, example_content, "blob".into(), true).unwrap();

    assert_eq!(
        execute(&fs, "blob".into(), "5c7f7d".into()).unwrap(),
        "awesome contents yo"
    );
}

#[test]
fn test_execute_wrong_first_parameter() {
    let fs = FileSystem::access();

    assert_eq!(
        execute(
            &fs,
            "NON EXISTING PARAMETER".into(),
            "5c7f7d83d0da2baceb3789aaf457a699455992fe".into()
        )
        .unwrap_err(),
        "fatal: papyrus cat-file first parameter can only receive one of (-t, blob, tree)"
    );
}

#[test]
fn test_execute_existing_file_type() {
    use crate::sub_commands::hash_object;
    let mut fs = FileSystem::access();

    let example_content = b"awesome contents yo";

    hash_object::execute(&mut fs, example_content, "blob".into(), true).unwrap();

    assert_eq!(
        execute(
            &fs,
            "-t".into(),
            "5c7f7d83d0da2baceb3789aaf457a699455992fe".into()
        )
        .unwrap(),
        "blob"
    );
}
