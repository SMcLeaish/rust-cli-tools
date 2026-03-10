use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust `cat`
struct Args {
    /// Input files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// Number of lines
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}

fn run(_args: Args) -> Result<()> {
    Ok(())
}
