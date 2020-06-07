use crate::fs::{FileSystem, Fs};
use crate::index::IndexEntry;
use crate::sub_commands::hash_object;

pub fn execute(fs: &mut FileSystem) -> Result<String, String> {
    let index_path = format!("{}/.papyrus/index", fs.current_directory());

    let index_content = fs.get_file_contents_as_bytes(&index_path.into()).unwrap();
    let index_entries = IndexEntry::parse_from_file(&index_content)?;

    let mut tree_entries = vec![];

    for index_entry in index_entries {
        tree_entries.push("");
    }

    hash_object::execute(fs, tree_entries.join("").into(), "tree".into(), true)
}
