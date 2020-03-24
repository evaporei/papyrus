use std::path::PathBuf;
use structopt::StructOpt;

mod hash_object;

#[derive(StructOpt, Debug)]
pub enum SubCommands {
    HashObject { file_name: PathBuf },
}

impl SubCommands {
    pub fn execute(self) {
        match self {
            SubCommands::HashObject { file_name } => {
                hash_object::execute(file_name);
            }
        }
    }
}
