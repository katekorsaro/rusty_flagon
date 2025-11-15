use crate::*;

#[derive(Debug, Default, PartialEq, Clone, clap::Parser)]
pub struct O {
    #[command(subcommand)]
    pub command: cli::Command,
}
