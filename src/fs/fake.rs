use crate::fs::Fs;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct FakeFs {
    files: HashMap<PathBuf, String>,
    directories: HashSet<PathBuf>,
    current_directory: String,
}

impl Fs for FakeFs {
    fn access() -> Self {
        let mut files = HashMap::new();

        files.insert(
            "example.txt".to_string().into(),
            "contents\nanother line".to_string(),
        );

        let current_directory = "/Users/jack/cool_project".to_string();

        let mut directories = HashSet::new();

        directories.insert(current_directory.clone().into());

        Self {
            files,
            directories,
            current_directory,
        }
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
    fn create_directory<P: AsRef<Path> + Eq>(&mut self, path: &P) {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        self.directories.insert(pathbuf);
    }
    fn remove_directory<P: AsRef<Path> + Eq>(&mut self, path: &P) {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        self.directories.remove(&pathbuf);
    }
    fn path_exists<P: AsRef<OsStr> + ?Sized + Eq + AsRef<Path>>(&self, path: &P) -> bool {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);

        self.directories.contains(&pathbuf) || self.files.contains_key(&pathbuf)
    }
    fn current_directory(&self) -> String {
        self.current_directory.clone()
    }
    fn create_file<P: AsRef<Path> + Eq>(&mut self, path: &P) {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        self.files.insert(pathbuf, "".to_string());
    }
}
