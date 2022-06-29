use clap::{Parser, Subcommand};
use tsplcore::{Tsvm, tsvminit, execmain};
use std::fs;

mod basicassembly;
mod tsplcore;
mod bytecode;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// compile from tspl basicassembly and execute it
    Run {
        /// File to run
        #[clap(long, value_parser)]
        file: String,
        
        /// Show vm debug output
        #[clap(short, long, action)]
        debug: bool,
    },

    /// execute from a tspl bytecode
    Exec {
        /// File to execute
        #[clap(long, value_parser)]
        file: String,

        /// Show vm debug output
        #[clap(short, long, action)]
        debug: bool,
    },
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run {file, debug} => {
            let mut vm: Tsvm = tsvminit();
            basicassembly::assembler(file, &mut vm);
            execmain(&mut vm, *debug);
        },
        Commands::Exec {file, debug} => {
            let mut vm: Tsvm = tsvminit();
        
            let contents = fs::read_to_string(file)
                .expect("Something went wrong reading the file");
            vm.mem = bytecode::program_parser(&contents).unwrap().1;
            execmain(&mut vm, *debug);
        }
    }
}