use std::env::temp_dir;

use clap::{Args, Parser, Subcommand};
use squid::{install::Installer, uninstall::Uninstaller};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    use SubCommands::*;
    match cli.subcommand {
        Install(args) => {
            Installer::connect_to_repository(&args.name, args.version)
                .await
                .find_package()
                .await
                .get_package_version()
                .await
                .pre_download()
                .await
                .download()
                .await
                .extract()
                .await
                .unwrap()
                .await
                .pre_install()
                .await
                .install()
                .await
                .clean_up()
                .await;
        }
        Uninstall(args) => {
            let uninstaller = Uninstaller::find_package(args.name.as_str()).await.unwrap();
            uninstaller.unlink().await;
            uninstaller.delete().await;
        }
        Continue => {
            println!("Fetching data...");
            let file = temp_dir().join("Continue.toml");
            if !file.exists() {
                println!("No session found. Aborting.");
                return;
            }
            let mut installer: Installer =
                toml::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
            println!("Found session, resuming");
            installer.resume().await;
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Install a package
    Install(InstallArgs),
    /// Uninstall a package
    Uninstall(UninstallArgs),
    /// Continue with the installaion
    Continue,
}

#[derive(Args)]
struct InstallArgs {
    /// The name of the package to be installed.
    name: String,
    /// An optional version of the package to be installed.
    #[clap(short, long)]
    version: Option<String>, // Todo: Change String to a proper version struct
}

#[derive(Args)]
struct UninstallArgs {
    /// The name of the package to be removed.
    name: String,
}
