use clap::{Parser, Subcommand};
use timetracker::storage;

#[derive(Parser)]
#[command(name = "tt")]
#[command(about = "A simple time tracker")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start { description: String },
    Status,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let filename = "timetracker.toml";

    match cli.command {
        Commands::Start { description } => {
            let mut tracker = storage::load_from_file(filename)?;
            tracker.start_timer(description.clone());
            storage::save_to_file(&tracker, filename)?;
            println!("Started timer: {}", description);
        }

        Commands::Status => {
            let tracker = storage::load_from_file(filename)?;
            match tracker.get_running_timer() {
                Some(entry) => println!("Timer running: {}", entry.description),
                None => println!("No timer currently running"),
            }
        }
    }
    Ok(())
}
