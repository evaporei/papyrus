use crate::fs::Fs;
use std::cmp::Eq;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{create_dir_all, remove_dir_all, OpenOptions};
use std::fs::{read_dir, read_to_string, File};
use std::io::{Read, Write};
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
        create_dir_all(path).unwrap();
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
    fn create_file<P: AsRef<Path> + Eq>(&mut self, path: &P) {
        File::create(path).unwrap();
    }
    fn write_file<P: AsRef<Path> + Eq>(&mut self, path: &P, contents: &[u8]) {
        let mut file = OpenOptions::new().write(true).open(path).unwrap();
        file.write_all(contents).unwrap();
    }
    fn get_file_contents_as_bytes(&self, file_name: &PathBuf) -> Result<Vec<u8>, String> {
        let mut buffer = Vec::new();

        let mut f = File::open(file_name)
            .map_err(|err| format!("fatal: Cannot open '{:?}': {}", file_name, err))?;
        f.read_to_end(&mut buffer).unwrap();

        Ok(buffer)
    }
    fn get_directory_files_starting_with(
        &self,
        directory: &PathBuf,
        file_name: &PathBuf,
    ) -> Vec<PathBuf> {
        read_dir(directory)
            .unwrap()
            .map(Result::unwrap)
            .filter(|a| a.path().is_file())
            .filter(|a| {
                a.path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(&file_name.to_str().unwrap()[2..])
            })
            .map(|a| a.path())
            .collect::<Vec<PathBuf>>()
    }
}
