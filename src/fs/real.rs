use crate::fs::Fs;
use std::cmp::Eq;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::fs::{create_dir, remove_dir_all};
use std::path::{Path, PathBuf};

pub struct RealFs;

impl Fs for RealFs {
    fn access() -> Self {
        Self
    }
    fn get_file_contents(&self, file_name: &PathBuf) -> Result<String, String> {
        read_to_string(&file_name)
            .map_err(|err| format!("fatal: Cannot open '{:?}': {}", file_name, err))
    }
    fn create_directory<P: AsRef<Path> + Eq>(&mut self, path: &P) {
        create_dir(path).unwrap();
    }
    fn remove_directory<P: AsRef<Path> + Eq>(&mut self, path: &P) {
        remove_dir_all(path).unwrap();
    }
    fn path_exists<P: AsRef<OsStr> + ?Sized + Eq + AsRef<Path>>(&self, path: &P) -> bool {
        Path::new(path).exists()
    }
    fn current_directory(&self) -> String {
        let current_directory_pathbuf = current_dir().unwrap();
        let current_directory = current_directory_pathbuf.to_str().unwrap();
        current_directory.to_string()
    }
}
