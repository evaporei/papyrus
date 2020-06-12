use crate::fs::Fs;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::str::from_utf8;

pub type FileSystem = FakeFs;
pub type FileMetadata = FakeFileMetadata;

pub struct FakeFileMetadata;

impl MetadataExt for FakeFileMetadata {
    fn dev(&self) -> u64 {
        16777220
    }
    fn ino(&self) -> u64 {
        21517258
    }
    fn mode(&self) -> u32 {
        unimplemented!();
    }
    fn nlink(&self) -> u64 {
        unimplemented!();
    }
    fn uid(&self) -> u32 {
        501
    }
    fn gid(&self) -> u32 {
        20
    }
    fn rdev(&self) -> u64 {
        unimplemented!();
    }
    fn size(&self) -> u64 {
        unimplemented!();
    }
    fn atime(&self) -> i64 {
        unimplemented!();
    }
    fn atime_nsec(&self) -> i64 {
        unimplemented!();
    }
    fn mtime(&self) -> i64 {
        1591510158
    }
    fn mtime_nsec(&self) -> i64 {
        unimplemented!();
    }
    fn ctime(&self) -> i64 {
        1591510158
    }
    fn ctime_nsec(&self) -> i64 {
        unimplemented!();
    }
    fn blksize(&self) -> u64 {
        unimplemented!();
    }
    fn blocks(&self) -> u64 {
        unimplemented!();
    }
}

impl FakeFileMetadata {
    pub fn len(&self) -> u64 {
        279
    }
    pub fn permissions(&self) -> FakePermissions {
        FakePermissions
    }
}

pub struct FakePermissions;

impl PermissionsExt for FakePermissions {
    fn mode(&self) -> u32 {
        33188
    }
    fn set_mode(&mut self, _mode: u32) {
        unimplemented!();
    }
    fn from_mode(_mode: u32) -> Self {
        unimplemented!();
    }
}

pub struct FakeFs {
    files: HashMap<PathBuf, Vec<u8>>,
    directories: HashSet<PathBuf>,
    current_directory: String,
}

impl Fs for FakeFs {
    fn access() -> Self {
        let mut files = HashMap::new();

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
        let p = PathBuf::from(file_name);
        let pathbuf = if p.is_absolute() {
            p
        } else {
            let mut pathbuf = PathBuf::from(self.current_directory());
            pathbuf.push(file_name);
            pathbuf
        };
        match self.files.get(&pathbuf) {
            Some(contents) => Ok(from_utf8(contents).unwrap().to_string()),
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
        let p = PathBuf::from(path);
        let pathbuf = if p.is_absolute() {
            p
        } else {
            let mut pathbuf = PathBuf::from(self.current_directory());
            pathbuf.push(path);
            pathbuf
        };

        self.directories.contains(&pathbuf) || self.files.contains_key(&pathbuf)
    }
    fn current_directory(&self) -> String {
        self.current_directory.clone()
    }
    fn create_file<P: AsRef<Path> + Eq>(&mut self, path: &P) {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        self.files.insert(pathbuf, vec![]);
    }
    fn write_file<P: AsRef<Path> + Eq>(&mut self, path: &P, contents: &[u8]) {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        self.files.insert(pathbuf, contents.to_vec());
    }
    fn get_file_contents_as_bytes(&self, file_name: &PathBuf) -> Result<Vec<u8>, String> {
        match self.files.get(file_name) {
            Some(contents) => Ok(contents.to_vec()),
            None => Err(format!(
                "fatal: Cannot open '{:?}': No such file or directory (os error 2)",
                file_name
            )),
        }
    }
    fn get_directory_files_starting_with(
        &self,
        directory: &PathBuf,
        file_name: &PathBuf,
    ) -> Vec<PathBuf> {
        let mut full_file_path = directory.clone();
        full_file_path.push(&file_name.to_str().unwrap()[2..]);

        self.files
            .keys()
            .filter(|k| {
                k.to_str()
                    .unwrap()
                    .starts_with(&full_file_path.to_str().unwrap())
            })
            .map(|p| p.clone())
            .collect::<Vec<PathBuf>>()
    }
    fn metadata<P: AsRef<Path>>(&self, _path: &P) -> Result<FileMetadata, String> {
        // needs to check if path exists
        Ok(FakeFileMetadata)
    }
}
