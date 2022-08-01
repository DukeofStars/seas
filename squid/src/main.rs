use clap::{Args, Parser, Subcommand};
use squid::{install::Installer, uninstall::Uninstaller};

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    use SubCommands::*;
    match cli.subcommand {
        Install(args) => {
            Installer::connect_to_repository()
                .await
                .find_package(&args.name)
                .await
                .get_package_version(args.version)
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
            let uninstaller = Uninstaller::find_package(args.name.as_str()).await?;
            uninstaller.unlink().await;
            uninstaller.delete().await;
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Install(InstallArgs),
    Uninstall(UninstallArgs),
}

#[derive(Args)]
struct InstallArgs {
    name: String,
    #[clap(short, long)]
    version: Option<String>, // Todo: Change String to a proper version struct
}

#[derive(Args)]
struct UninstallArgs {
    name: String,
}
