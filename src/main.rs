mod vm;

use std::{fs, io, path::PathBuf, process::exit};

use clap::Parser;
use colored::Colorize;

use crate::vm::VirtualMachine;

// Manage CLI arguments
#[derive(Debug, Parser)]
#[command(name = "Unremarkable StackMachine")]
#[command(about = "Interpreter for USM", long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Path to file you want to interpret",
        value_name = "FILE"
    )]
    file_path: PathBuf,

    #[arg(
        short,
        long,
        help = "Enable debug mode. Machine state will be printed after each step. Press [Enter] to advance machine state",
        value_name = "DEBUG_MODE",
        action = clap::ArgAction::SetTrue,
    )]
    debug: bool,
}

fn main() {
    let (file_path, debug) = {
        let cli = Cli::parse();
        (cli.file_path, cli.debug)
    };

    let contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", format!("Cannot read file {file_path:?}: {e:}").red());
            exit(1)
        }
    };

    println!("Programm to interpret: {contents}");

    // Create a new language runtime
    let mut vm = match VirtualMachine::try_new(contents) {
        Ok(vm) => vm,
        Err(e) => {
            println!("{}", format!("Failed interpreting input: {e:}").red());
            exit(1)
        }
    };
    if debug {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal
    }
    while {
        if debug {
            println!("{vm:}");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect(&"Failed to read from stdin".red());

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal
        }
        vm.step();

        !vm.endstate()
    } {}
}
