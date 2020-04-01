use std::path::PathBuf;
use structopt::StructOpt;

mod hash_object;
mod init;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    Init,
    HashObject { file_name: PathBuf },
}

impl SubCommand {
    pub fn execute(self) {
        let output = match self {
            Self::Init => init::execute(),
            Self::HashObject { file_name } => hash_object::execute(file_name),
        };

        match output {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
    }
}
