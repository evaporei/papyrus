use crate::fs::{FileSystem, Fs};
use crate::index::IndexEntry;
use crate::sub_commands::hash_object;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::PathBuf;
use std::str::from_utf8;

pub fn execute(fs: &mut FileSystem, files: Vec<PathBuf>) -> Result<String, String> {
    let index_path = format!("{}/.papyrus/index", fs.current_directory());

    let index_file_exists = fs.path_exists(&index_path);

    let mut entries: Vec<IndexEntry> = if !index_file_exists {
        fs.create_file(&index_path);
        vec![]
    } else {
        let index_content = fs
            .get_file_contents_as_bytes(&index_path.clone().into())
            .unwrap();

        let index_entries = IndexEntry::parse_from_file(&index_content)?;

        index_entries
            .into_iter()
            .filter(|ie| {
                let path = from_utf8(&ie.path).unwrap();
                !files.contains(&PathBuf::from(path))
            })
            .collect()
    };

    for file in files {
        let file_str = file.to_str().unwrap();

        if !fs.path_exists(&file) {
            return Err(format!(
                "fatal: pathspec '{}' did not match any files",
                file_str
            ));
        }

        let sha1 = hash_object::execute(fs, file.clone(), "blob".into(), true)?;

        let metadata = fs.metadata(&file)?;
        let permissions = metadata.permissions();

        let mut entry = IndexEntry::default();

        let ctime_bytes = metadata.ctime().to_be_bytes();
        entry
            .ctime_s
            .copy_from_slice(&ctime_bytes[ctime_bytes.len() - 4..]);

        entry.ctime_n.copy_from_slice(&[0, 0, 0, 0]);

        let mtime_bytes = metadata.mtime().to_be_bytes();
        entry
            .mtime_s
            .copy_from_slice(&mtime_bytes[mtime_bytes.len() - 4..]);

        entry.mtime_n.copy_from_slice(&[0, 0, 0, 0]);

        let dev_bytes = metadata.dev().to_be_bytes();
        entry.dev.copy_from_slice(&dev_bytes[dev_bytes.len() - 4..]);

        let ino_bytes = metadata.ino().to_be_bytes();
        entry.ino.copy_from_slice(&ino_bytes[ino_bytes.len() - 4..]);

        entry
            .mode
            .copy_from_slice(&permissions.mode().to_be_bytes());
        entry.uid.copy_from_slice(&metadata.uid().to_be_bytes());
        entry.gid.copy_from_slice(&metadata.gid().to_be_bytes());

        let size_bytes = metadata.len().to_be_bytes();
        entry
            .size
            .copy_from_slice(&size_bytes[size_bytes.len() - 4..]);

        let sha1_bytes = sha1.as_bytes();
        let mut output: Vec<u8> = vec![];

        for byte_pair in sha1_bytes.chunks(2) {
            let b = from_utf8(byte_pair).unwrap();
            let b = u8::from_str_radix(&b, 16).unwrap();
            output.push(b);
        }

        entry.sha1.copy_from_slice(&output);

        let flags_bytes = file_str.len().to_be_bytes();
        entry
            .flags
            .copy_from_slice(&flags_bytes[flags_bytes.len() - 2..]);

        for p in file_str.as_bytes() {
            entry.path.push(*p);
        }

        entries.push(entry);
    }

    entries.sort();

    let new_index_file_content = IndexEntry::parse_into_file(entries);

    fs.write_file(&index_path, &new_index_file_content);

    Ok("".to_string())
}

#[test]
fn execute_when_index_file_doesnt_exist() {
    use crate::sub_commands::init;
    let mut fs = FileSystem::access();

    init::execute(&mut fs).unwrap();

    let file1_path = format!("{}/file1.txt", fs.current_directory());
    let file1_content = "cool content";

    fs.create_file(&file1_path);
    fs.write_file(&file1_path, file1_content.as_bytes());

    let file2_path = format!("{}/file2.txt", fs.current_directory());
    let file2_content = "moar content";

    fs.create_file(&file2_path);
    fs.write_file(&file2_path, file2_content.as_bytes());

    assert_eq!(
        execute(&mut fs, vec![file1_path.into(), file2_path.into()]).unwrap(),
        ""
    );

    let index_path = format!("{}/.papyrus/index", fs.current_directory());
    fs.path_exists(&index_path);
    assert_eq!(
        fs.get_file_contents_as_bytes(&index_path.into()).unwrap(),
        vec![
            68, 73, 82, 67, 0, 0, 0, 2, 0, 0, 0, 2, 94, 220, 132, 142, 0, 0, 0, 0, 94, 220, 132,
            142, 0, 0, 0, 0, 1, 0, 0, 4, 1, 72, 83, 202, 0, 0, 129, 164, 0, 0, 1, 245, 0, 0, 0, 20,
            0, 0, 1, 23, 191, 128, 49, 139, 33, 118, 113, 189, 25, 174, 69, 72, 73, 111, 200, 119,
            128, 11, 1, 80, 0, 34, 47, 85, 115, 101, 114, 115, 47, 106, 97, 99, 107, 47, 99, 111,
            111, 108, 95, 112, 114, 111, 106, 101, 99, 116, 47, 102, 105, 108, 101, 49, 46, 116,
            120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 94, 220, 132, 142, 0, 0, 0, 0, 94, 220, 132, 142, 0,
            0, 0, 0, 1, 0, 0, 4, 1, 72, 83, 202, 0, 0, 129, 164, 0, 0, 1, 245, 0, 0, 0, 20, 0, 0,
            1, 23, 35, 246, 130, 118, 105, 228, 56, 49, 222, 248, 167, 173, 147, 80, 105, 200, 189,
            65, 130, 97, 0, 34, 47, 85, 115, 101, 114, 115, 47, 106, 97, 99, 107, 47, 99, 111, 111,
            108, 95, 112, 114, 111, 106, 101, 99, 116, 47, 102, 105, 108, 101, 50, 46, 116, 120,
            116, 0, 0, 0, 0, 0, 0, 0, 0, 15, 184, 97, 178, 88, 219, 181, 208, 154, 96, 108, 32,
            152, 105, 86, 208, 186, 172, 150, 62
        ]
    );
}

#[test]
fn execute_when_index_file_already_exists() {
    use crate::sub_commands::init;
    let mut fs = FileSystem::access();

    init::execute(&mut fs).unwrap();

    let file1_path = format!("{}/file1.txt", fs.current_directory());
    let file1_content = "cool content";

    fs.create_file(&file1_path);
    fs.write_file(&file1_path, file1_content.as_bytes());

    let file2_path = format!("{}/file2.txt", fs.current_directory());
    let file2_content = "changed! content";

    fs.create_file(&file2_path);
    fs.write_file(&file2_path, file2_content.as_bytes());

    let index_path = format!("{}/.papyrus/index", fs.current_directory());
    let index_content = [
        68, 73, 82, 67, 0, 0, 0, 2, 0, 0, 0, 2, 94, 220, 132, 142, 0, 0, 0, 0, 94, 220, 132, 142,
        0, 0, 0, 0, 1, 0, 0, 4, 1, 72, 83, 202, 0, 0, 129, 164, 0, 0, 1, 245, 0, 0, 0, 20, 0, 0, 1,
        23, 191, 128, 49, 139, 33, 118, 113, 189, 25, 174, 69, 72, 73, 111, 200, 119, 128, 11, 1,
        80, 0, 34, 47, 85, 115, 101, 114, 115, 47, 106, 97, 99, 107, 47, 99, 111, 111, 108, 95,
        112, 114, 111, 106, 101, 99, 116, 47, 102, 105, 108, 101, 49, 46, 116, 120, 116, 0, 0, 0,
        0, 0, 0, 0, 0, 94, 220, 132, 142, 0, 0, 0, 0, 94, 220, 132, 142, 0, 0, 0, 0, 1, 0, 0, 4, 1,
        72, 83, 202, 0, 0, 129, 164, 0, 0, 1, 245, 0, 0, 0, 20, 0, 0, 1, 23, 35, 246, 130, 118,
        105, 228, 56, 49, 222, 248, 167, 173, 147, 80, 105, 200, 189, 65, 130, 97, 0, 34, 47, 85,
        115, 101, 114, 115, 47, 106, 97, 99, 107, 47, 99, 111, 111, 108, 95, 112, 114, 111, 106,
        101, 99, 116, 47, 102, 105, 108, 101, 50, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 15,
        184, 97, 178, 88, 219, 181, 208, 154, 96, 108, 32, 152, 105, 86, 208, 186, 172, 150, 62,
    ];

    fs.create_file(&index_path);
    fs.write_file(&index_path, &index_content);

    assert_eq!(
        execute(&mut fs, vec![file1_path.into(), file2_path.into()]).unwrap(),
        ""
    );

    let index_path = format!("{}/.papyrus/index", fs.current_directory());
    fs.path_exists(&index_path);
    assert_eq!(
        fs.get_file_contents_as_bytes(&index_path.into()).unwrap(),
        vec![
            68, 73, 82, 67, 0, 0, 0, 2, 0, 0, 0, 2, 94, 220, 132, 142, 0, 0, 0, 0, 94, 220, 132,
            142, 0, 0, 0, 0, 1, 0, 0, 4, 1, 72, 83, 202, 0, 0, 129, 164, 0, 0, 1, 245, 0, 0, 0, 20,
            0, 0, 1, 23, 191, 128, 49, 139, 33, 118, 113, 189, 25, 174, 69, 72, 73, 111, 200, 119,
            128, 11, 1, 80, 0, 34, 47, 85, 115, 101, 114, 115, 47, 106, 97, 99, 107, 47, 99, 111,
            111, 108, 95, 112, 114, 111, 106, 101, 99, 116, 47, 102, 105, 108, 101, 49, 46, 116,
            120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 94, 220, 132, 142, 0, 0, 0, 0, 94, 220, 132, 142, 0,
            0, 0, 0, 1, 0, 0, 4, 1, 72, 83, 202, 0, 0, 129, 164, 0, 0, 1, 245, 0, 0, 0, 20, 0, 0,
            1, 23, 236, 62, 127, 142, 227, 218, 246, 50, 102, 100, 32, 44, 9, 37, 91, 108, 85, 180,
            100, 18, 0, 34, 47, 85, 115, 101, 114, 115, 47, 106, 97, 99, 107, 47, 99, 111, 111,
            108, 95, 112, 114, 111, 106, 101, 99, 116, 47, 102, 105, 108, 101, 50, 46, 116, 120,
            116, 0, 0, 0, 0, 0, 0, 0, 0, 221, 94, 59, 30, 252, 54, 175, 16, 154, 198, 40, 39, 27,
            28, 189, 29, 90, 246, 136, 138
        ]
    );
}

#[test]
fn execute_when_one_of_passing_files() {
    use crate::sub_commands::init;
    let mut fs = FileSystem::access();

    init::execute(&mut fs).unwrap();

    let file1_path = format!("{}/file1.txt", fs.current_directory());
    let file1_content = "cool content";

    fs.create_file(&file1_path);
    fs.write_file(&file1_path, file1_content.as_bytes());

    let file2_path = format!("{}/file2.txt", fs.current_directory());

    assert_eq!(
        execute(&mut fs, vec![file1_path.into(), file2_path.into()]).unwrap_err(),
        "fatal: pathspec '/Users/jack/cool_project/file2.txt' did not match any files"
    );
}
