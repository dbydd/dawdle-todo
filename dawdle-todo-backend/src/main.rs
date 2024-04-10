#![feature(fs_try_exists)]

use clap::{arg, Command};
use clap::{Parser, Subcommand};

mod configurations;
mod modifiers;
mod task;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    test: Option<Test>,
}

#[derive(Subcommand)]
enum Test {
    /// Add a number
    Add {
        #[arg(short, long)]
        num: u16,
    },
    /// Sub a number
    Sub {
        #[arg(short, long)]
        num: u16,
    },
}

fn main() {}
