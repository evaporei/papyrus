use crate::fs::Fs;
use std::cmp::Eq;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{create_dir_all, remove_dir_all, OpenOptions};
use std::fs::{read_dir, read_to_string, File};
use std::fs::{Metadata, Permissions};
use std::io::{Read, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

pub type FileSystem = RealFs;
pub type FileMetadata = RealFileMetadata;

pub struct RealFileMetadata(Metadata);

impl MetadataExt for RealFileMetadata {
    fn dev(&self) -> u64 {
        self.0.dev()
    }
    fn ino(&self) -> u64 {
        self.0.ino()
    }
    fn mode(&self) -> u32 {
        self.0.mode()
    }
    fn nlink(&self) -> u64 {
        self.0.nlink()
    }
    fn uid(&self) -> u32 {
        self.0.uid()
    }
    fn gid(&self) -> u32 {
        self.0.gid()
    }
    fn rdev(&self) -> u64 {
        self.0.rdev()
    }
    fn size(&self) -> u64 {
        self.0.size()
    }
    fn atime(&self) -> i64 {
        self.0.atime()
    }
    fn atime_nsec(&self) -> i64 {
        self.0.atime_nsec()
    }
    fn mtime(&self) -> i64 {
        self.0.mtime()
    }
    fn mtime_nsec(&self) -> i64 {
        self.0.mtime_nsec()
    }
    fn ctime(&self) -> i64 {
        self.0.ctime()
    }
    fn ctime_nsec(&self) -> i64 {
        self.0.ctime_nsec()
    }
    fn blksize(&self) -> u64 {
        self.0.blksize()
    }
    fn blocks(&self) -> u64 {
        self.0.blocks()
    }
}

impl RealFileMetadata {
    pub fn len(&self) -> u64 {
        self.0.len()
    }
    pub fn permissions(&self) -> RealPermissions {
        RealPermissions(self.0.permissions())
    }
}

pub struct RealPermissions(Permissions);

impl PermissionsExt for RealPermissions {
    fn mode(&self) -> u32 {
        self.0.mode()
    }
    fn set_mode(&mut self, mode: u32) {
        self.0.set_mode(mode);
    }
    fn from_mode(mode: u32) -> Self {
        Self(Permissions::from_mode(mode))
    }
}

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
    fn metadata<P: AsRef<Path>>(&self, path: &P) -> Result<FileMetadata, String> {
        let m = std::fs::metadata(path).map_err(|e| format!("{}", e))?;

        Ok(RealFileMetadata(m))
    }
}
