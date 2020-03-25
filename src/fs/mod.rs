use std::path::PathBuf;

#[cfg(not(test))]
mod real;

#[cfg(test)]
mod fake;

pub trait Fs {
    fn access() -> Self;
    fn get_file_contents(&self, file_name: &PathBuf) -> Result<String, String>;
}

#[cfg(not(test))]
pub type FileSystem = real::RealFs;

#[cfg(test)]
pub type FileSystem = fake::FakeFs;
