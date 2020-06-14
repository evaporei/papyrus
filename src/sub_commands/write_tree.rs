use crate::fs::{FileSystem, Fs};
use crate::index::IndexEntry;
use crate::sub_commands::hash_object;
use std::convert::TryInto;
use std::str::from_utf8;

pub fn execute(fs: &mut FileSystem) -> Result<String, String> {
    let index_path = format!("{}/.papyrus/index", fs.current_directory());

    let index_content = fs.get_file_contents_as_bytes(&index_path.into()).unwrap();
    let index_entries = IndexEntry::parse_from_file(&index_content)?;

    let mut tree_entries = vec![];

    for index_entry in index_entries {
        let mode = u32::from_be_bytes(index_entry.mode.try_into().unwrap());
        let path = from_utf8(&index_entry.path).unwrap();

        let mode_path = format!("{:o} {}", mode, path);

        let mut tree_entry = vec![];

        let mut mode_path_bytes = mode_path.as_bytes().to_vec();

        tree_entry.append(&mut mode_path_bytes);
        tree_entry.append(&mut b"\x00".to_vec());
        tree_entry.append(&mut index_entry.sha1.to_vec());

        tree_entries.push(tree_entry);
    }

    hash_object::execute(fs, &tree_entries.concat(), "tree".into(), true)
}

#[test]
fn execute_successfully() {
    let mut fs = FileSystem::access();

    let index_path = format!("{}/.papyrus/index", fs.current_directory());

    fs.create_file(&index_path);
    fs.write_file(
        &index_path,
        &[
            68, 73, 82, 67, 0, 0, 0, 2, 0, 0, 0, 1, 94, 220, 132, 142, 0, 0, 0, 0, 94, 220, 132,
            142, 0, 0, 0, 0, 1, 0, 0, 4, 1, 72, 83, 202, 0, 0, 129, 164, 0, 0, 1, 245, 0, 0, 0, 20,
            0, 0, 1, 23, 22, 168, 14, 57, 131, 236, 189, 112, 58, 63, 67, 88, 88, 140, 254, 8, 213,
            33, 154, 44, 0, 10, 67, 97, 114, 103, 111, 46, 116, 111, 109, 108, 0, 0, 0, 0, 0, 0, 0,
            0, 221, 161, 67, 7, 244, 26, 34, 221, 145, 12, 6, 131, 203, 40, 226, 145, 244, 89, 40,
            157,
        ],
    );

    assert_eq!(
        execute(&mut fs).unwrap(),
        "7d11a85a54c02af57434e2bcd5ea7d7ea303e4ac"
    );

    assert!(fs.path_exists(&format!("{}/.papyrus/objects/7d", fs.current_directory())));
    assert!(fs.path_exists(&format!(
        "{}/.papyrus/objects/7d/11a85a54c02af57434e2bcd5ea7d7ea303e4ac",
        fs.current_directory()
    )));
    let full_file_path = format!(
        "{}/.papyrus/objects/7d/11a85a54c02af57434e2bcd5ea7d7ea303e4ac",
        fs.current_directory()
    )
    .into();
    assert_eq!(
        fs.get_file_contents_as_bytes(&full_file_path).unwrap(),
        vec![
            120, 156, 1, 46, 0, 209, 255, 116, 114, 101, 101, 32, 51, 56, 0, 49, 48, 48, 54, 52,
            52, 32, 67, 97, 114, 103, 111, 46, 116, 111, 109, 108, 0, 22, 168, 14, 57, 131, 236,
            189, 112, 58, 63, 67, 88, 88, 140, 254, 8, 213, 33, 154, 44, 74, 83, 15, 188
        ]
    );
}
