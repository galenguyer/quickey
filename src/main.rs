use clap::{Parser, ValueEnum};

mod algorithms;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum, default_value_t = Algorithm::Base58)]
    algorithm: Algorithm,

    #[arg(short, long, default_value_t = 32)]
    bytes: usize,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    #[default]
    Base58,
    Base64,
    Hex,
}

fn main() {
    let cli = Cli::parse();

    match cli.algorithm {
        Algorithm::Base58 => println!("{}", algorithms::base58(cli.bytes)),
        Algorithm::Base64 => println!("{}", algorithms::base64(cli.bytes)),
        Algorithm::Hex => println!("{}", algorithms::hex(cli.bytes)),
    }
}
