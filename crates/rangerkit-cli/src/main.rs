use clap::{Parser, Subcommand};
use rangerkit_core::SpiritManifest;
use serde_json;
use nu_ansi_term::Color;
use anyhow::Result;
use anyhow::Context;

mod tui;
use tui::commune::run_commune_tui;


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

    /// Summon a spirit by name.
    Summon {
        /// The name of the spirit to summon.
        spirit_name: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Spirits { subcommand } => match subcommand {
            SpiritCommands::Commune { output_format } => 
                commune_with_spirits(output_format),
            
            SpiritCommands::Summon { spirit_name } =>
                summon_spirit(spirit_name),
            
        },
    }
}

/// Handles communing with the spirits. Lists known spirits and their abilities.
fn commune_with_spirits(output_format: &str) -> Result<()> {
    let manifest = SpiritManifest::default();

    match output_format {
        "json" => {
            let json = serde_json::to_string_pretty(&manifest)
              .expect("Failed to serialize spirits to JSON.");
          println!("{}", json);
          Ok(())
        }
        "text" => {
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
            Ok(())
        }
        _ => {
            // For anything else, run the TUI
            run_commune_tui().context("failed to launch spirit gathering TUI.")
        }
    }
}


/// Handles summoning a spirit by name.
fn summon_spirit(spirit_name: &str) -> Result<()> {
    let manifest = SpiritManifest::default();

    let spirit = manifest.spirits.iter()
      .find(|s| s.name.to_lowercase() == spirit_name.to_lowercase());

    match spirit {
        Some(spirit) => {
            println!("\n✨ You summon the spirit:\n");
            println!("{}  {}", Color::Yellow.paint(&spirit.glyph), Color::Cyan.bold().paint(&spirit.name));
            for ability in &spirit.abilities {
                println!("    {} {}", Color::Green.paint("-"), ability.name);
                println!("       {}", ability.description);
            }
            println!("\n🌿 The spirit watches over your trail.\n");
            Ok(())
        }
        None => {
            println!("\n🌫️  No spirit by that name answered your call.\n");
            Ok(())
        }
    }
}
