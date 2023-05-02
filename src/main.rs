use clap::{Parser, ValueEnum};

mod base58;

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
}

fn main() {
    let cli = Cli::parse();

    match cli.algorithm {
        Algorithm::Base58 => println!("{}", base58::base58(cli.bytes)),
    }
}
