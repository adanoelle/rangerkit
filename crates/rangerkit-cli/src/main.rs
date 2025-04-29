use clap::{Parser, Subcommand};
use rangerkit_core::SpiritManifest;


/// RangerKit CLI: Walk the trails with spirit companions.
#[derive(Parser)]
#[command(name = "rangerkit")]
#[command(about = "🌲 Summon spirits. Map waystones.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Work with your spirit companions.
    Spirits {
        #[command(subcommand)]
        subcommand: SpiritCommands,
    }
}

#[derive(Subcommand)]
enum SpiritCommands {
    /// Commune with the spirits and see who walks beside you.
    Commune,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Spirits { subcommand } => match subcommand {
            SpiritCommands::Commune => {
                commune_with_spirits();
            }
        },
    }
}

/// Handles communing with the spirits. Lists known spirits and their abilities.
fn commune_with_spirits() {
    let manifest = SpiritManifest::default();

    println!("\n🌿 The spirits gather around you:\n");

    for spirit in manifest.spirits {
        println!("{} {}", spirit.glyph, spirit.name);
        for ability in spirit.abilities {
            println!("    - {}: {}", ability.name, ability.description);
        }
        println!(); // Soft spacing
    }
}
