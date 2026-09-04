use clap::{Parser, Subcommand};
use apman_core::application;

#[derive(Parser)]
struct Cli {
	#[command(subcommand)]
	command: Commands
}
#[derive(Subcommand)]
enum Commands {
	Version,
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Version => {
			println!("apman version {}", application::VERSION);
			println!("commit {}", application::GIT_HASH);
		}
	}
}
