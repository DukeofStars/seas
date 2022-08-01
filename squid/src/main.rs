use clap::{Args, Parser, Subcommand};
use squid::install::Installer;

#[tokio::main]
async fn main() {
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
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Install(InstallArgs),
}

#[derive(Args)]
struct InstallArgs {
    name: String,
    #[clap(short, long)]
    version: Option<String>, // Todo: Change String to a proper version struct
}
