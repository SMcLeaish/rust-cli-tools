use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Target files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// Print n lines
    #[arg(short('n'), long("lines"), default_value = "10",
    value_parser = clap::value_parser!(u64).range(1..))]
    lines: u64,
    /// Print n bytes
    #[arg(
        short('c'), long("bytes"), conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args)
}
