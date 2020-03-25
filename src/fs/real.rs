use crate::fs::Fs;
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct RealFs;

impl Fs for RealFs {
    fn access() -> Self {
        Self
    }
    fn get_file_contents(&self, file_name: &PathBuf) -> Result<String, String> {
        read_to_string(&file_name)
            .map_err(|err| format!("fatal: Cannot open '{:?}': {}", file_name, err))
    }
}
