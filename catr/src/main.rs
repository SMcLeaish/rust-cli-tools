use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// Target files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// Print line numbers
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,
    /// Print non-blank line numbers
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut line_number = 1;
                for (_, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        println!("{:>6}\t{line}", line_number);
                        line_number += 1;
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!("{line}")
                        } else {
                            println!("{:>6}\t{line}", line_number);
                            line_number += 1;
                        }
                    } else {
                        println!("{line}")
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
