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

        let sha1 = hash_object::execute(fs, file.clone(), true)?;
        let metadata = std::fs::metadata(file.clone()).unwrap();

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

        entry
            .sha1
            .copy_from_slice(&output);

        entry
            .flags
            .copy_from_slice(&(0b1010111111111111u16).to_be_bytes());

        for p in file_str.as_bytes() {
            entry.path.push(*p);
        }

        entries.push(entry);
    }

    entries.sort();

    Ok("".to_string())
}
