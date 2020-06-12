use crate::fs::{FileSystem, Fs};
use std::path::PathBuf;
use structopt::StructOpt;

pub mod add;
pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_files;
pub mod write_tree;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    Init,
    HashObject {
        file_name: PathBuf,
        #[structopt(short, long)]
        write: bool,
        #[structopt(default_value = "blob", short = "t", long = "type")]
        object_type: String,
    },
    CatFile(CatFile),
    LsFiles {
        #[structopt(short, long)]
        stage: bool,
    },
    Add {
        files: Vec<PathBuf>,
    },
    WriteTree,
}

#[derive(StructOpt, Debug)]
pub enum CatFile {
    Blob { file_name: String },
    Type { file_name: String },
}

impl SubCommand {
    pub fn execute(self) {
        let mut fs = FileSystem::access();

        let output = match self {
            Self::Init => init::execute(&mut fs),
            Self::HashObject {
                file_name,
                object_type,
                write,
            } => hash_object::execute(&mut fs, file_name, object_type, write),
            Self::CatFile(CatFile::Blob { file_name }) => {
                cat_file::execute(&fs, "blob".to_string(), file_name)
            }
            Self::CatFile(CatFile::Type { file_name }) => {
                cat_file::execute(&fs, "-t".to_string(), file_name)
            }
            Self::LsFiles { stage } => ls_files::execute(&fs, stage),
            Self::Add { files } => add::execute(&mut fs, files),
            Self::WriteTree => write_tree::execute(&mut fs),
        };

        match output {
            Ok(result) => {
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    }
}
