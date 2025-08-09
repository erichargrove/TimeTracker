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
    Stop,
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

        Commands::Stop => {
            let mut tracker = storage::load_from_file(filename)?;
            if tracker.stop_current_timer() {
                storage::save_to_file(&tracker, filename)?;
                println!("Timer stopped!");
            } else {
                println!("No timer currently running.");
            }
        }

        Commands::Status => {
            let tracker = storage::load_from_file(filename)?;
            match tracker.get_running_timer() {
                Some(entry) => {
                    let duration = entry.duration();
                    let hrs = duration.num_hours();
                    let min = duration.num_minutes() % 60;
                    let sec = duration.num_seconds() % 60;

                    println!("Timer running: {}", entry.description);
                    println!("Duration: {}h {}m {}s", hrs, min, sec);
                }
                None => println!("No timer currently running"),
            }
        }
    }
    Ok(())
}
