use clap::{Parser, Subcommand};
use rangerkit_core::SpiritManifest;
use serde_json;
use nu_ansi_term::Color;


/// RangerKit CLI: Walk the trails with spirit companions.
#[derive(Debug, Parser)]
#[command(name = "rangerkit")]
#[command(about = "🌲 Summon spirits. Map waystones.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Work with your spirit companions.
    Spirits {
        #[command(subcommand)]
        subcommand: SpiritCommands,
    }
}

#[derive(Debug, Subcommand)]
enum SpiritCommands {
    /// Commune with the spirits and see who walks beside you.
    Commune {
        /// Output format: "text" or "json".
        #[arg(long = "format", default_value = "text")]
        output_format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Spirits { subcommand } => match subcommand {
            SpiritCommands::Commune { output_format } => {
                commune_with_spirits(output_format);
            }
        },
    }
}

/// Handles communing with the spirits. Lists known spirits and their abilities.
fn commune_with_spirits(output_format: &str) {
    let manifest = SpiritManifest::default();

    match output_format {
        "json" => {
            let json = serde_json::to_string_pretty(&manifest)
              .expect("Failed to serialize spirits to JSON.");
          println!("{}", json);
        }
        "text" | _ => {
            println!("\n🌿 The spirits gather around you:\n");

            for spirit in manifest.spirits {
                println!("{}  {}", 
                    Color::Yellow.paint(&spirit.glyph), 
                    Color::Cyan.bold().paint(&spirit.name)
                );

                for ability in spirit.abilities {
                    println!("    {}: {}", 
                        Color::Green.paint(format!("- {}", ability.name)), 
                        ability.description
                    );
                }
                println!(); // Soft spacing
            }
        }
    }
}
