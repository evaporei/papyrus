use crate::fs::{FileSystem, Fs};
use std::path::PathBuf;
use structopt::StructOpt;

mod hash_object;
mod init;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    Init,
    HashObject {
        file_name: PathBuf,
        #[structopt(short, long)]
        write: bool,
    },
}

impl SubCommand {
    pub fn execute(self) {
        let mut fs = FileSystem::access();

        let output = match self {
            Self::Init => init::execute(&mut fs),
            Self::HashObject { file_name, write } => {
                hash_object::execute(&mut fs, file_name, write)
            }
        };

        match output {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
    }
}
