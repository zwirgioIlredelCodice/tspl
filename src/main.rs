use clap::{Parser, Subcommand};
use tsplcore::{Tsvm, tsvminit, execmain};

mod basicassembly;
mod tsplcore;

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
            basicassembly::assemblyfromfile(file, &mut vm);
            execmain(&mut vm, *debug);
        }
    }
}