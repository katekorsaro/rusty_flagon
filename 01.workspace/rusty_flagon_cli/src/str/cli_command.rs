use crate::*;

#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, clap::Subcommand)]
pub enum E {
    #[default]
    StdOut,
    File,
}
