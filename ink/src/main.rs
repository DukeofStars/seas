use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use clap::{Args, Parser, Subcommand};

use share::Global;
use squidserver::Message;

mod share;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialise cli
    let cli = Cli::parse();

    // Initialise globals
    let globals = Arc::new(Mutex::new(Global::default()));

    use Command::*;
    match cli.command {
        Install(_package_name) => {
            let ui_thread = tokio::spawn(ui::run_ui(globals.clone()));

            let _stream = squidserver::send(Message::Hello(15)).await?;
            println!("Successfully sent hello");

            globals.lock().unwrap().should_close = true;
            ui_thread.await?;
        }
        Hello => {
            println!("Hello!");
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Hello,
    /// Installs the specified package
    Install(InstallArgs),
}

#[derive(Args)]
struct InstallArgs {
    package_name: String,
}
