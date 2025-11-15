use clap::*;
use rusty_flagon_lib::*;
use std::*;

mod fun;
mod imp;
mod mcr;
mod str;
mod trt;
mod tst;

use crate::fun::print_character::run as print_character;

mod cli {
    pub use crate::str::cli::O as Cli;
    pub use crate::str::cli_command::E as Command;
}

mod handle {
    pub use crate::fun::handle_file::run as file;
    pub use crate::fun::handle_stdout::run as stdout;
}

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::StdOut => handle::stdout(),
        cli::Command::File => handle::file(),
    }
}
