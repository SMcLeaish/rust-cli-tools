use clap::{Arg, ArgAction, Command};

fn main() {
    let m = Command::new("echor")
        .author("Sean McLeaish <smcleaish@gmail.com>")
        .version("0.1.0")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();
    let text: Vec<String> = m.get_many("text").unwrap().cloned().collect();
    let omit_newline = m.get_flag("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
