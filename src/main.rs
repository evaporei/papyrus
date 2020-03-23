use structopt::StructOpt;
use papyrus::sub_commands::SubCommands;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    sub_commands: SubCommands,
}

fn main() {
    let opt = Opt::from_args();

    opt.sub_commands.execute();
}
