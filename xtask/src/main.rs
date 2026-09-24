//! `cargo xtask`: g-star repository tasks.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask", about = "g-star repository tasks")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the repository gates.
    Gate {
        /// Run only the named gate.
        #[arg(long)]
        only: Option<String>,
        /// Print the gate names in run order, and exit.
        #[arg(long)]
        list: bool,
    },
    /// Print the pinned version of an external tool (used by CI).
    ToolVersion {
        /// The tool's executable name, as keyed in `tools/versions.toml`.
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Gate { only, list } => {
            let gates = xtask::registry();
            if list {
                for gate in &gates {
                    println!("{}", gate.name);
                }
                return Ok(());
            }
            xtask::run_gates(&gates, only.as_deref()).map(drop)
        }
        Command::ToolVersion { name } => {
            let pins = xtask::tools::load_pins(&xtask::workspace::root()?)?;
            let Some(pin) = pins.get(&name) else {
                anyhow::bail!("tool `{name}` is not pinned in tools/versions.toml");
            };
            println!("{}", pin.version);
            Ok(())
        }
    }
}
