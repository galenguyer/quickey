use clap::{Parser, ValueEnum};

mod algorithms;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum, default_value_t = Algorithm::Base58)]
    algorithm: Algorithm,

    #[arg(short, long, default_value_t = 32)]
    bytes: usize,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    #[default]
    Base58,
    Base64,
    Hex,
}

impl core::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Algorithm::Base58 => write!(f, "base58"),
            Algorithm::Base64 => write!(f, "base64"),
            Algorithm::Hex => write!(f, "hex"),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("generating {} bytes using {}", cli.bytes, cli.algorithm);
    }

    match cli.algorithm {
        Algorithm::Base58 => println!("{}", algorithms::base58(cli.bytes)),
        Algorithm::Base64 => println!("{}", algorithms::base64(cli.bytes)),
        Algorithm::Hex => println!("{}", algorithms::hex(cli.bytes)),
    }
}
