use clap::{Parser, Subcommand};
use apman_core::application;
use apman_device::distro;
use apman_device::distro::OsType;
use apman_device::model::get_model_type;
use apman_device::network::{get_network_devices, remove_unimportant_devices};
#[derive(Parser)]
struct Cli {
	#[command(subcommand)]
	command: Commands
}
#[derive(Subcommand)]
enum Commands {
	Version,
	Device {
        #[command(subcommand)]
        action: DeviceAction,
    },
}

#[derive(Subcommand)]
enum DeviceAction {
    Info,
    List,
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Version => {
			println!("apman version {}", application::VERSION);
			println!("commit {}", application::GIT_HASH);
		},
		Commands::Device { action } => match action {
            DeviceAction::Info => {
				let osinf = distro::get_os_info();
				let modelinf = get_model_type();
				match osinf {
					OsType::Linux { distro, release, .. } => {
						println!("Distro: {}", release.pretty_name);
						println!("Base Distro: {}", release.name);
						println!("Board Model: {}", modelinf.as_str());
						println!("Detected network devices:");
						for item in remove_unimportant_devices(get_network_devices()) {
							println!("    {}", item);
						}
					}
					_ => {}
				}
            }
            DeviceAction::List => {
                println!("listing devices...");
            }
        },
	}
	
}
