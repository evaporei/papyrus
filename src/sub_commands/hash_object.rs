use crate::fs::{FileSystem, Fs};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use std::path::PathBuf;

fn create_sha1(input: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.input(&input[..]);
    hasher.result_str()
}

#[test]
fn test_create_sha1() {
    assert_eq!(
        create_sha1(b"blob 21\x00contents\nanother line"),
        "f9936bb09530fbc19a32568bde0738d9234037e4"
    );
}

fn zlib_compress(input: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input).unwrap();
    encoder.finish().unwrap()
}

#[test]
fn test_zlib_compress() {
    assert_eq!(
        zlib_compress(b"blob 21\x00contents\nanother line"),
        vec![
            120, 156, 75, 202, 201, 79, 82, 48, 50, 100, 72, 206, 207, 43, 73, 205, 43, 41, 230,
            74, 204, 203, 47, 201, 72, 45, 82, 200, 201, 204, 75, 5, 0, 148, 92, 10, 84
        ]
    );
}

pub fn execute(
    fs: &mut FileSystem,
    contents: &[u8],
    object_type: String,
    write: bool,
) -> Result<String, String> {
    let mut contents_len = String::new();

    for i in contents.len().to_string().chars() {
        contents_len.push(i);
    }

    let object_contents = [
        object_type.as_bytes(),
        b" ",
        &contents_len.as_bytes(),
        b"\x00",
        &contents,
    ]
    .concat();

    let sha1 = create_sha1(&object_contents);

    if write {
        let object_folder = &sha1[..2];
        let object_file = &sha1[2..];

        let mut absolute_folder_path = PathBuf::from(fs.current_directory());
        absolute_folder_path.push(".papyrus");
        absolute_folder_path.push("objects");
        absolute_folder_path.push(object_folder);

        if !fs.path_exists(&absolute_folder_path) {
            fs.create_directory(&absolute_folder_path);
        }

        let mut absolute_file_path = absolute_folder_path.clone();
        absolute_file_path.push(object_file);
        absolute_file_path.set_file_name(object_file);

        if !fs.path_exists(&absolute_file_path) {
            fs.create_file(&absolute_file_path);
            let compressed_object_contents = zlib_compress(&object_contents);

            fs.write_file(&absolute_file_path, &compressed_object_contents);
        }
    }

    Ok(sha1)
}

#[test]
fn test_execute_without_write() {
    let mut fs = FileSystem::access();

    let example_content = b"contents\nanother line";

    assert_eq!(
        execute(&mut fs, example_content, "blob".into(), false).unwrap(),
        "f9936bb09530fbc19a32568bde0738d9234037e4"
    );
}

#[test]
fn test_execute_with_write() {
    let mut fs = FileSystem::access();

    let example_content = b"contents\nanother line";

    assert_eq!(
        execute(&mut fs, example_content, "blob".into(), true).unwrap(),
        "f9936bb09530fbc19a32568bde0738d9234037e4"
    );

    assert!(fs.path_exists(&format!("{}/.papyrus/objects/f9", fs.current_directory())));
    assert!(fs.path_exists(&format!(
        "{}/.papyrus/objects/f9/936bb09530fbc19a32568bde0738d9234037e4",
        fs.current_directory()
    )));
    let full_file_path = format!(
        "{}/.papyrus/objects/f9/936bb09530fbc19a32568bde0738d9234037e4",
        fs.current_directory()
    )
    .into();
    assert_eq!(
        fs.get_file_contents_as_bytes(&full_file_path).unwrap(),
        vec![
            120, 156, 75, 202, 201, 79, 82, 48, 50, 100, 72, 206, 207, 43, 73, 205, 43, 41, 230,
            74, 204, 203, 47, 201, 72, 45, 82, 200, 201, 204, 75, 5, 0, 148, 92, 10, 84
        ]
    );
}
