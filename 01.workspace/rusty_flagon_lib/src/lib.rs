//! A library for generating characters for OSE Classic roleplaying game.
#![warn(missing_docs)]

mod fun;
mod imp;
mod mcr;
mod str;
mod trt;
mod tst;

use rand::*;

use crate::str::roller::O as Roller;

/// The alignment of a character.
pub use crate::str::alignment::E as Alignment;
/// A builder for creating characters.
pub use crate::str::builder::O as Builder;
/// A character.
pub use crate::str::character::O as Character;
/// The class of a character.
pub use crate::str::class::E as Class;
/// An error that can occur when building a character.
pub use crate::str::failed_to::E as FailedTo;
