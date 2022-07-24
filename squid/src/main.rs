use std::{fs, os, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use compression::prelude::{DecodeExt, GZipDecoder};
use trauma::{download::Download, downloader::DownloaderBuilder};

use directories::ProjectDirs;
use jellyfish::{request::Repository, Package, PackageVersion};

use colored::*;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let data_path = data_path();

    use SubCommands::*;
    match cli.subcommand {
        Install(args) => {
            // Connect to the repository
            let repository = "http://localhost:8000/jellyfish/"; // TODO: load this from config file and allow for multiple repositories
            let repo = Repository::connect(repository);

            // Get pacakge from server
            let package: Package = repo
                .get_package(&args.name)
                .await
                .expect("Failed to find requested package from the remote");

            let package_version: &PackageVersion = match args.version {
                Some(version) => package
                    .versions
                    .iter()
                    .filter(|x| x.version == version)
                    .next()
                    .expect("The requested version was not found"),
                None => package
                    .versions
                    .last()
                    .expect("This package has no versions"),
            };

            let relative_path = PathBuf::from("dist").join(&package.name).join(format!(
                "{}-{}.tar.gz",
                package_version.version, package_version.id
            ));

            let download_path = repository.clone().to_string() + relative_path.to_str().unwrap();

            // Ensure temp path exists
            let temp_path = temp_path();
            if !temp_path.exists() {
                fs::create_dir_all(&temp_path).unwrap();
            }

            // Download the package
            println!("{} {}", "Downloading".green().bold(), download_path);
            let downloads = vec![Download::try_from(download_path.as_str()).unwrap()]; // In the future, get all the downloads to be performed and do all of them at once.
            let downloader = DownloaderBuilder::new()
                .directory(temp_path.clone())
                .build();
            downloader.download(&downloads).await;

            // Extract the package
            let file_path = temp_path.join(format!(
                "{}-{}.tar.gz",
                package_version.version, package_version.id
            ));
            println!(
                "{} {}",
                "Extracting".green().bold(),
                file_path.to_str().unwrap()
            );
            let file = fs::read(&file_path).unwrap();
            // Remove the temp file.
            fs::remove_file(&file_path).unwrap();
            let uncompressed = file
                .iter()
                .cloned()
                .decode(&mut GZipDecoder::new())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            // Unwrap tar ball
            let mut tar_ball = tar::Archive::new(uncompressed.as_slice());
            tar_ball
                .unpack(data_path.join(&package.name).as_path())
                .unwrap();

            // Install the package
            println!("{} {}", "Installing".green().bold(), package.name);
            let bin_folder = data_path.join(&package.name).join("bin");
            if bin_folder.exists() && bin_folder.is_dir() {
                for entry in fs::read_dir(bin_folder).unwrap() {
                    let entry = entry.unwrap();
                    println!(
                        "{} {}",
                        "Linking".blue().bold(),
                        entry.file_name().to_str().unwrap()
                    );
                    // Create symlink from the file to the global bin folder.
                    let target = data_path.join(".bin").join(entry.file_name());
                    let target_root = target.parent().unwrap();
                    if !target_root.exists() {
                        fs::create_dir_all(target_root).unwrap();
                    }
                    // If the target already exists, ask the user if they would like to skip this file or replace it.
                    if target.exists() {
                        let expanded = target.canonicalize().unwrap();
                        // LOL spaghetti code. basically ../../ then the name of the file(directory).
                        let package = expanded
                            .parent()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap();
                        println!(
                            "{}: File {} already exists from package {}",
                            "Error".red().bold(),
                            target.to_str().unwrap(),
                            package
                        );
                        println!("{}", "Would you like to replace it? (y/n)");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        if input.trim() == "y" {
                            fs::remove_file(&target).unwrap();
                        } else {
                            continue;
                        }
                    }
                    #[cfg(windows)]
                    {
                        if entry.path().is_dir() {
                            os::windows::fs::symlink_dir(entry.path(), target).unwrap();
                        } else {
                            os::windows::fs::symlink_file(entry.path(), target).unwrap();
                        }
                    }

                    #[cfg(not(windows))]
                    os::unix::fs::symlink(entry.path(), target).unwrap();
                }
            } else {
                println!(
                    "{}: {}",
                    "Warning".yellow().bold(),
                    "No installation target found"
                );
            }
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

pub fn data_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("", "", "squid").unwrap();
    project_dirs.data_dir().to_path_buf()
}

pub fn temp_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("", "", "squid").unwrap();
    project_dirs.cache_dir().to_path_buf()
}
