use crate::fs::{FileSystem, Fs};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::convert::TryInto;

#[derive(Default, PartialEq)]
struct IndexEntry {
    ctime_s: [u8; 4],
    ctime_n: [u8; 4],
    mtime_s: [u8; 4],
    mtime_n: [u8; 4],
    dev: [u8; 4],
    ino: [u8; 4],
    mode: [u8; 4],
    uid: [u8; 4],
    gid: [u8; 4],
    size: [u8; 4],
    sha1: [u8; 20],
    flags: [u8; 2],
    path: Vec<u8>,
}

pub fn execute(fs: &FileSystem, stage: bool) -> Result<String, String> {
    let index_path = format!("{}/.papyrus/index", fs.current_directory());

    if !fs.path_exists(&index_path) {
        return Ok("".to_string());
    }

    let index_content = fs.get_file_contents_as_bytes(&index_path.into()).unwrap();

    let entries = parse_index_file_content(&index_content)?;

    Ok(format_index_entries(entries, stage))
}

fn parse_index_file_content(index_content: &[u8]) -> Result<Vec<IndexEntry>, String> {
    let mut hasher = Sha1::new();
    let index_of_checksum = index_content.len() - 20;
    hasher.input(&index_content[..index_of_checksum]);
    let size = hasher.output_bytes();
    let mut sha1_bytes = vec![0; size];
    hasher.result(&mut sha1_bytes);

    let checksum: Vec<u8> = index_content.iter().rev().take(20).rev().map(|a| *a).collect();

    // sanity check of checksum
    if sha1_bytes != checksum {
        return Err("error: bad index file sha1 signature\nfatal: index file corrupt".to_string());
    }

    let header = &index_content[..12];

    let signature = &header[..4];
    // sanity check of signature
    if signature != b"DIRC" {
        return Err("error: bad signature\nfatal: index file corrupt".to_string());
    }

    let version = &header[4..8];
    // sanity check of version
    if version != &[0, 0, 0, 2] {
        return Err("error: bad version\nfatal: index file corrupt".to_string());
    }

    let number_of_entries = &header[8..12];

    let entry_data = &index_content[12..index_of_checksum];
    let quantity = u32::from_be_bytes(number_of_entries.try_into().unwrap());
    let mut entries: Vec<IndexEntry> = vec![];
    let mut i = 0;
    let mut count = 1;

    while i + 62 < entry_data.len() && count <= quantity {
        count += 1;
        let fields_end = i + 62;
        let fields = &entry_data[i..fields_end];
        let path_end = entry_data.iter().skip(fields_end).position(|a| *a == b'\x00').unwrap() + fields_end;
        let path = &entry_data[fields_end..path_end];

        let mut entry = IndexEntry::default();

        entry.ctime_s.copy_from_slice(&fields[..4]);
        entry.ctime_n.copy_from_slice(&fields[4..8]);
        entry.mtime_s.copy_from_slice(&fields[8..12]);
        entry.mtime_n.copy_from_slice(&fields[12..16]);
        entry.dev.copy_from_slice(&fields[16..20]);
        entry.ino.copy_from_slice(&fields[20..24]);
        entry.mode.copy_from_slice(&fields[24..28]);
        entry.uid.copy_from_slice(&fields[28..32]);
        entry.gid.copy_from_slice(&fields[32..36]);
        entry.size.copy_from_slice(&fields[36..40]);

        entry.sha1.copy_from_slice(&fields[40..60]);
        entry.flags.copy_from_slice(&fields[60..62]);

        for p in path {
            entry.path.push(*p);
        }

        entries.push(entry);

        let entry_length = ((62 + path.len() + 8) / 8) * 8;
        i += entry_length;
    }

    Ok(entries)
}

fn format_index_entries(entries: Vec<IndexEntry>, stage: bool) -> String {
    let mut output = String::new();

    for entry in &entries {
        if stage {
            let flags = u16::from_be_bytes(entry.flags.try_into().unwrap());
            let stage = (flags >> 12) & 3;

            let mode = u32::from_be_bytes(entry.mode.try_into().unwrap());
            output.push_str(&format!("{:o} ", mode));

            for s in entry.sha1.iter() {
                output.push_str(&format!("{:02x}", s));
            }

            output.push_str(&format!(" {:?}\t", stage));
        }

        let path = std::str::from_utf8(&entry.path).unwrap();
        output.push_str(&format!("{}", path));

        if let Some(last) = entries.last() {
            if last != entry {
                output.push_str("\n");
            }
        }
    }

    output
}
