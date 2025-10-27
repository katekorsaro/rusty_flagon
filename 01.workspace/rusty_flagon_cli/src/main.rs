use rusty_flagon_lib::*;
use std::*;

mod fun;
mod imp;
mod mcr;
mod str;
mod trt;
mod tst;

fn main() {
    let mut builder = Builder::new();
    let character = builder.build();
    match character {
        Ok(value) => println!("{value}"),
        Err(_) => println!("An error occurred while generating a new character"),
    }
}
