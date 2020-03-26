use papyrus::sub_commands::SubCommand;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

fn main() {
    let opt = Opt::from_args();

    opt.sub_command.execute();
}
