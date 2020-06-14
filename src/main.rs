use papyrus::sub_commands::SubCommand;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

fn main() {
    let opt = Opt::from_args();

    let output = opt.sub_command.execute();

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
