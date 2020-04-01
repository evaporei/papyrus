use std::path::PathBuf;
use structopt::StructOpt;
use crate::fs::{FileSystem, Fs};

mod hash_object;
mod init;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    Init,
    HashObject { file_name: PathBuf },
}

impl SubCommand {
    pub fn execute(self) {
        let mut fs = FileSystem::access();

        let output = match self {
            Self::Init => init::execute(&mut fs),
            Self::HashObject { file_name } => hash_object::execute(&mut fs, file_name),
        };

        match output {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
    }
}
