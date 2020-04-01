use std::env::current_dir;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;

pub fn execute() -> Result<String, String> {
    let current_directory_pathbuf = current_dir().unwrap();
    let current_directory = current_directory_pathbuf.to_str().unwrap();
    let papyrus_path = format!("{}/.papyrus/", current_directory);

    let message = if !Path::new(&papyrus_path).exists() {
        create_dir(&papyrus_path).unwrap();
        create_dir(format!("{}objects", &papyrus_path)).unwrap();
        format!("Initialized empty Papyrus repository in {}", papyrus_path)
    } else {
        remove_dir_all(&papyrus_path).unwrap();
        create_dir(&papyrus_path).unwrap();
        format!(
            "Reinitialized existing Papyrus repository in {}",
            papyrus_path
        )
    };

    Ok(message)
}
