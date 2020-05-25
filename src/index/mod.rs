use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::cmp::Ordering;
use std::convert::TryInto;

#[derive(Default, PartialEq, Eq)]
pub struct IndexEntry {
    pub ctime_s: [u8; 4],
    pub ctime_n: [u8; 4],
    pub mtime_s: [u8; 4],
    pub mtime_n: [u8; 4],
    pub dev: [u8; 4],
    pub ino: [u8; 4],
    pub mode: [u8; 4],
    pub uid: [u8; 4],
    pub gid: [u8; 4],
    pub size: [u8; 4],
    pub sha1: [u8; 20],
    pub flags: [u8; 2],
    pub path: Vec<u8>,
}

impl PartialOrd for IndexEntry {
    fn partial_cmp(&self, other: &IndexEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IndexEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl IndexEntry {
    pub fn parse_from_file(index_content: &[u8]) -> Result<Vec<Self>, String> {
        let header = &index_content[..12];

        let signature = &header[..4];
        // sanity check of signature
        if signature != b"DIRC" {
            return Err("error: bad signature\nfatal: index file corrupt".to_string());
        }

        let version = &header[4..8];
        // sanity check of version
        if version != [0, 0, 0, 2] {
            return Err("error: bad version\nfatal: index file corrupt".to_string());
        }

        let number_of_entries = &header[8..12];

        let mut hasher = Sha1::new();
        let index_of_checksum = index_content.len() - 20;
        hasher.input(&index_content[..index_of_checksum]);
        let size = hasher.output_bytes();
        let mut sha1_bytes = vec![0; size];
        hasher.result(&mut sha1_bytes);

        let checksum: Vec<u8> = index_content.iter().rev().take(20).rev().copied().collect();

        // sanity check of checksum
        if sha1_bytes != checksum {
            return Err(
                "error: bad index file sha1 signature\nfatal: index file corrupt".to_string(),
            );
        }

        let entry_data = &index_content[12..index_of_checksum];
        let quantity = u32::from_be_bytes(number_of_entries.try_into().unwrap());
        let mut entries: Vec<Self> = vec![];
        let mut i = 0;
        let mut count = 1;

        while i + 62 < entry_data.len() && count <= quantity {
            count += 1;
            let fields_end = i + 62;
            let fields = &entry_data[i..fields_end];
            let path_end = entry_data
                .iter()
                .skip(fields_end)
                .position(|a| *a == b'\x00')
                .unwrap()
                + fields_end;
            let path = &entry_data[fields_end..path_end];

            let mut entry = Self::default();

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

    pub fn format_index_entries(entries: Vec<Self>, stage: bool) -> String {
        let mut output = String::new();

        for entry in &entries {
            if stage {
                let flags = u16::from_be_bytes(entry.flags.try_into().unwrap());
                let stage = (flags >> 12) & 3;

                let mode = u32::from_be_bytes(entry.mode.try_into().unwrap());
                output.push_str(&format!("{:o} ", mode));

                for s in &entry.sha1 {
                    output.push_str(&format!("{:02x}", s));
                }

                output.push_str(&format!(" {:?}\t", stage));
            }

            let path = std::str::from_utf8(&entry.path).unwrap();
            output.push_str(&path);

            if let Some(last) = entries.last() {
                if last != entry {
                    output.push_str("\n");
                }
            }
        }

        output
    }

    pub fn parse_into_file(entries: Vec<Self>) -> Vec<u8> {
        let mut index_file_bytes = vec![];

        let mut header = {
            let signature = b"DIRC";
            let version = &[0, 0, 0, 2];
            let number_of_entries = &entries.len().to_be_bytes();

            [signature, version, &number_of_entries[number_of_entries.len() - 4..]].concat()
        };

        index_file_bytes.append(&mut header);

        for entry in entries {
            let mut entry_bytes = [
                entry.ctime_s,
                entry.ctime_n,
                entry.mtime_s,
                entry.mtime_n,
                entry.dev,
                entry.ino,
                entry.mode,
                entry.uid,
                entry.gid,
                entry.size,
            ].concat().to_vec();

            entry_bytes.append(&mut entry.sha1.to_vec());
            entry_bytes.append(&mut entry.flags.to_vec());
            entry_bytes.append(&mut entry.path.to_vec());

            let length = ((62 + entry.path.len() + 8) / 8) * 8;

            for _ in 0..(length - 62 - entry.path.len()) {
                entry_bytes.push(b'\x00');
            }

            index_file_bytes.append(&mut entry_bytes);
        }

        let mut hasher = Sha1::new();
        hasher.input(&index_file_bytes[..]);
        let size = hasher.output_bytes();
        let mut sha1_bytes = vec![0; size];
        hasher.result(&mut sha1_bytes);

        index_file_bytes.append(&mut sha1_bytes);

        index_file_bytes
    }
}
