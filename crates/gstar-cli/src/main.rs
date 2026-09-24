//! `gstar`: the g-star toolchain.

use clap::Parser;

/// The g-star toolchain.
#[derive(Parser)]
#[command(name = "gstar", version, about = "The g-star toolchain")]
struct Cli {}

fn main() {
    let Cli {} = Cli::parse();
}
