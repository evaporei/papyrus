use crate::fs::{FileSystem, Fs};

pub fn execute(fs: &mut FileSystem) -> Result<String, String> {
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

#[test]
fn test_execute_non_initialized() {
    let mut fs = FileSystem::access();

    let message = execute(&mut fs).unwrap();

    assert_eq!(
        message,
        "Initialized empty Papyrus repository in /Users/jack/cool_project/.papyrus/"
    );

    assert!(fs.path_exists(&format!("{}/.papyrus/", fs.current_directory())));
    assert!(fs.path_exists(&format!("{}/.papyrus/objects", fs.current_directory())));
}

#[test]
fn test_execute_already_initialized() {
    let mut fs = FileSystem::access();

    fs.create_directory(&format!("{}/.papyrus/", fs.current_directory()));

    let message = execute(&mut fs).unwrap();

    assert_eq!(
        message,
        "Reinitialized existing Papyrus repository in /Users/jack/cool_project/.papyrus/"
    );

    assert!(fs.path_exists(&format!("{}/.papyrus/", fs.current_directory())));
}
