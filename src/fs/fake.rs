use crate::fs::Fs;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct FakeFs {
    files: HashMap<PathBuf, String>,
}

impl Fs for FakeFs {
    fn access() -> Self {
        let mut files = HashMap::new();

        files.insert(
            "example.txt".to_string().into(),
            "contents\nanother line".to_string(),
        );

        Self { files }
    }
    fn get_file_contents(&self, file_name: &PathBuf) -> Result<String, String> {
        match self.files.get(file_name) {
            Some(contents) => Ok(contents.to_string()),
            None => Err(format!(
                "fatal: Cannot open '{:?}': No such file or directory (os error 2)",
                file_name
            )),
        }
    }
}
