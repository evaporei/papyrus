use crate::fs::{FileSystem, Fs};
use std::path::PathBuf;
use structopt::StructOpt;

pub mod cat_file;
pub mod hash_object;
pub mod init;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    Init,
    HashObject {
        file_name: PathBuf,
        #[structopt(short, long)]
        write: bool,
    },
    CatFile {
        file_type: String,
        file_name: String,
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
            Self::CatFile {
                file_type,
                file_name,
            } => cat_file::execute(&fs, file_type, file_name),
        };

        match output {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
    }
}
