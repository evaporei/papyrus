use crate::fs::{FileSystem, Fs};

pub fn execute() -> Result<String, String> {
    let mut fs = FileSystem::access();
    let current_directory = fs.current_directory();
    let papyrus_path = format!("{}/.papyrus/", current_directory);

    let message = if !fs.path_exists(&papyrus_path) {
        fs.create_directory(&papyrus_path);
        fs.create_directory(&format!("{}objects", &papyrus_path));
        format!("Initialized empty Papyrus repository in {}", papyrus_path)
    } else {
        fs.remove_directory(&papyrus_path);
        fs.create_directory(&papyrus_path);
        format!(
            "Reinitialized existing Papyrus repository in {}",
            papyrus_path
        )
    };

    Ok(message)
}
