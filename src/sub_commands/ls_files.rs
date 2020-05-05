use crate::fs::{FileSystem, Fs};
use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn execute(fs: &mut FileSystem, stage: bool) -> Result<String, String> {
    if !stage {
        return Err("fatal: ls-files without --stage is not implemented yet".to_string());
    }

    let index_path = format!("{}/.papyrus/index", fs.current_directory());

    if !fs.path_exists(&index_path) {
        fs.create_file(&index_path);
    }

    let index_content = fs.get_file_contents_as_bytes(&index_path.into()).unwrap();

    let mut hasher = Sha1::new();
    let index_of_checksum = index_content.len() - 20;
    hasher.input(&index_content[..index_of_checksum]);
    let size = hasher.output_bytes();
    let mut sha1_bytes = vec![0; size];
    hasher.result(&mut sha1_bytes);

    let checksum: Vec<u8> = index_content.iter().rev().take(20).rev().map(|a| *a).collect();

    // sanity check of checksum
    assert_eq!(sha1_bytes, checksum);

    let header = &index_content[..12];

    let signature = &header[..4];
    // sanity check of signature
    assert_eq!(signature, b"DIRC");

    let version = &header[4..8];
    // sanity check of version
    assert_eq!(version, &[0, 0, 0, 2]);

    Ok("nothing".to_string())
}
