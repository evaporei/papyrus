use std::path::PathBuf;
use structopt::StructOpt;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::File;
use std::io::Read;

#[derive(StructOpt, Debug)]
pub enum SubCommands {
    HashObject { file_name: PathBuf },
}

impl SubCommands {
    pub fn execute(&self) {
        match self {
            SubCommands::HashObject { file_name } => {
                let mut file = File::open(file_name).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                let mut hasher = Sha1::new();
                hasher.input_str(&contents);
                let hex = hasher.result_str();
                println!("{}", hex);
            }
        }
    }
}
