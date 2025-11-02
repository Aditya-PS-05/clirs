use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust Version of Echo
struct Args {
    /// Input Text
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print Newline
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
