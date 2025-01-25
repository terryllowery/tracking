use clap::Parser;
use clap::Subcommand;

use error_stack::Result;

#[derive(Debug, thiserror::Error)]
#[error("a CLI error occurred")]

pub struct CliError;
/// Welcome to the time tracking application to track time spent on tasks.\n
/// This has been brought to you\n
/// by the Rust Programming Language.\n
/// @{author}
/// @version
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Start tracking time
    Start,
    // Stop,
    // Report,
}

pub fn run() -> Result<(), CliError> {

    let args = Cli::parse();
    match args.command {
        Command::Start => {
            println!("Starting the time tracking application");
        }
        
    }


    Ok(())
}