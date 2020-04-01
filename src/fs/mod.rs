use std::cmp::Eq;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[cfg(not(test))]
mod real;

#[cfg(test)]
mod fake;

pub trait Fs {
    fn access() -> Self;
    fn get_file_contents(&self, file_name: &PathBuf) -> Result<String, String>;
    fn create_directory<P: AsRef<Path> + Eq>(&mut self, path: &P);
    fn remove_directory<P: AsRef<Path> + Eq>(&mut self, path: &P);
    fn path_exists<P: AsRef<OsStr> + ?Sized + Eq + AsRef<Path>>(&self, path: &P) -> bool;
    fn current_directory(&self) -> String;
}

#[cfg(not(test))]
pub type FileSystem = real::RealFs;

#[cfg(test)]
pub type FileSystem = fake::FakeFs;
