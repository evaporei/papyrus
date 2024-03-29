use std::cmp::Eq;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[cfg(not(test))]
mod real;

#[cfg(not(test))]
pub use real::{FileMetadata, FileSystem};

#[cfg(test)]
mod fake;

#[cfg(test)]
pub use fake::{FileMetadata, FileSystem};

pub trait Fs {
    fn access() -> Self;
    fn get_file_contents(&self, file_name: &PathBuf) -> Result<String, String>;
    fn create_directory<P: AsRef<Path> + Eq>(&mut self, path: &P);
    fn remove_directory<P: AsRef<Path> + Eq>(&mut self, path: &P);
    fn path_exists<P: AsRef<OsStr> + ?Sized + Eq + AsRef<Path>>(&self, path: &P) -> bool;
    fn current_directory(&self) -> String;
    fn create_file<P: AsRef<Path> + Eq>(&mut self, path: &P);
    fn write_file<P: AsRef<Path> + Eq>(&mut self, path: &P, contents: &[u8]);
    fn get_file_contents_as_bytes(&self, file_name: &PathBuf) -> Result<Vec<u8>, String>;
    fn get_directory_files_starting_with(
        &self,
        directory: &PathBuf,
        file_name: &PathBuf,
    ) -> Vec<PathBuf>;
    fn metadata<P: AsRef<Path>>(&self, path: &P) -> Result<FileMetadata, String>;
}
