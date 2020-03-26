use std::path::PathBuf;
use structopt::StructOpt;

mod hash_object;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    HashObject { file_name: PathBuf },
}

impl SubCommand {
    pub fn execute(self) {
        let output = match self {
            Self::HashObject { file_name } => hash_object::execute(file_name),
        };

        match output {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
    }
}
