use clap::{Parser, Subcommand};

#[derive(Parser)]
// clap lee automáticamente name, version, authors y about de tu Cargo.toml
#[command(name = "vic")]
#[command(version)]
#[command(about = "Vicode CLI - Validated Infrastructure from Code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check the current version
    Version,
    /// Validate the infrastructure graph
    Validate,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version) => {
            // Usamos la función de nuestra propia lib
            println!("vic version {}", vicode::version());
        }
        Some(Commands::Validate) => {
            println!("Validating infrastructure... (Mock)");
        }
        None => {
            println!("Vicode CLI. Use 'vic --help' for commands.");
        }
    }
}
