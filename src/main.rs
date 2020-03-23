use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    sub_commands: SubCommands,
}

#[derive(StructOpt, Debug)]
enum SubCommands {
    HashObject { object: PathBuf },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
