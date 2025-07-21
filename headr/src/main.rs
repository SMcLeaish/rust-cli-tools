use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Target files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// Print n lines [default: 10]
    #[arg(short('n'), long("lines"), default_value = "10")]
    lines: u64,
    /// Print n bytes
    #[arg(short('b'), long("bytes"), num_args(0..))]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args)
}
