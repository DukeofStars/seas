use std::{fs, os, path::PathBuf};

use colored::Colorize;
use compression::prelude::{DecodeExt, GZipDecoder};
use directories::ProjectDirs;
use jellyfish::{request::Repository, Package, PackageVersion};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use trauma::{download::Download, downloader::DownloaderBuilder};

#[derive(Default)]
pub enum InstallStages {
    #[default]
    None,
    ConnectToRepository,
    FindPackage,
    GetPackageVersion,
    PreDownload,
    Download,
    Extract,
    Unwrap,
    PreInstall,
    Install,
    Cleanup,
}

pub struct Installer {
    pub stage: InstallStages,
    pub repo: Repository,
    pub package: Package,
    pub package_version: PackageVersion,
    pub file_path: PathBuf,
    pub relative_path: PathBuf,
    pub download_path: String,
    pub uncompressed: Vec<u8>,
    pub bin_folder: PathBuf,
}

impl Installer {
    pub async fn set_stage(&mut self, stage: InstallStages) {
        self.stage = stage;

        let path = temp_path().join("Continue.toml");
    }

    pub async fn connect_to_repository() -> Self {
        let mut me = Self {
            package: Package {
                author: Default::default(),
                name: Default::default(),
                friendly_name: Default::default(),
                versions: Default::default(),
            },
            package_version: PackageVersion {
                id: guid_create::GUID::rand(),
                version: "0.0.0".to_string(),
                required: Default::default(),
                dependencies: Default::default(),
            },
            stage: InstallStages::None,
            repo: Repository {
                url: "".to_string(),
            },
            file_path: Default::default(),
            relative_path: Default::default(),
            download_path: Default::default(),
            uncompressed: Default::default(),
            bin_folder: Default::default(),
        };
        me.stage = InstallStages::ConnectToRepository;
        // Connect to the repository
        let repository = "http://localhost:8000/jellyfish/"; // TODO: load this from config file and allow for multiple repositories
        me.repo = Repository::connect(repository).await;

        me
    }

    pub async fn find_package(&mut self, name: &str) -> &mut Self {
        self.stage = InstallStages::FindPackage;
        // Get pacakge from server
        self.package = self
            .repo
            .get_package(name)
            .await
            .expect("Failed to find requested package from the remote");

        self
    }

    pub async fn get_package_version(&mut self, version: Option<String>) -> &mut Self {
        self.stage = InstallStages::GetPackageVersion;
        self.package_version = match version {
            Some(version) => self
                .package
                .versions
                .iter()
                .filter(|x| x.version == version)
                .next()
                .expect("The requested version was not found"),
            None => self
                .package
                .versions
                .last()
                .expect("This package has no versions"),
        }
        .clone();

        self
    }

    pub async fn pre_download(&mut self) -> &mut Self {
        self.stage = InstallStages::PreDownload;
        self.relative_path = PathBuf::from("dist").join(&self.package.name).join(format!(
            "{}-{}.tar.gz",
            self.package_version.version, self.package_version.id
        ));

        self.download_path =
            self.repo.url.clone().to_string() + self.relative_path.to_str().unwrap();

        // Ensure temp path exists
        let temp_path = temp_path();
        if !temp_path.exists() {
            fs::create_dir_all(&temp_path).unwrap();
        }

        self
    }

    pub async fn download(&mut self) -> &mut Self {
        self.stage = InstallStages::Download;
        // Download the package
        println!("{} {}", "Downloading".green().bold(), self.download_path);
        let downloads = vec![Download::try_from(self.download_path.as_str()).unwrap()]; // In the future, get all the downloads to be performed and do all of them at once.
        let downloader = DownloaderBuilder::new()
            .directory(temp_path().clone())
            .build();
        downloader.download(&downloads).await;

        self
    }

    pub async fn extract(&mut self) -> &mut Self {
        self.stage = InstallStages::Extract;
        // Extract the package
        self.file_path = temp_path().join(format!(
            "{}-{}.tar.gz",
            self.package_version.version, self.package_version.id
        ));
        println!(
            "{} {}",
            "Extracting".green().bold(),
            self.file_path.to_str().unwrap()
        );
        let file = fs::read(&self.file_path).unwrap();

        self.uncompressed = file
            .iter()
            .cloned()
            .decode(&mut GZipDecoder::new())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        self
    }

    pub async fn unwrap(&mut self) -> &mut Self {
        self.stage = InstallStages::Unwrap;
        // Unwrap tar ball
        let mut tar_ball = tar::Archive::new(self.uncompressed.as_slice());
        tar_ball
            .unpack(data_path().join(&self.package.name).as_path())
            .unwrap();

        self
    }

    pub async fn pre_install(&mut self) -> &mut Self {
        self.stage = InstallStages::PreInstall;
        println!("{} {}", "Installing".green().bold(), self.package.name);
        self.bin_folder = data_path().join(&self.package.name).join("bin");
        if !(self.bin_folder.exists() && self.bin_folder.is_dir()) {
            println!(
                "{}: {}",
                "Warning".yellow().bold(),
                "No installation target found"
            );
        }

        self
    }

    pub async fn install(&mut self) -> &mut Self {
        self.stage = InstallStages::Install;
        let vec_entries =
            Vec::from_iter(fs::read_dir(self.bin_folder.clone()).unwrap().into_iter());
        vec_entries.par_iter().for_each(|entry| {
            let entry = entry.as_ref().unwrap();
            println!(
                "{} {}",
                "Linking".blue().bold(),
                entry.file_name().to_str().unwrap()
            );
            // Create symlink from the file to the global bin folder.
            let target = self.bin_folder.join(entry.file_name());
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
                print!("{}", "Would you like to replace it? (y/n) ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "y" {
                    fs::remove_file(&target).unwrap();
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
                } else {
                    println!("{}", "Skipping...".blue().bold());
                }
            } else {
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
        });

        self
    }

    pub async fn clean_up(&mut self) -> &mut Self {
        self.stage = InstallStages::Cleanup;
        // Remove the temp file.
        fs::remove_file(&self.file_path).unwrap();

        self
    }
}

pub fn data_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("", "", "squid").unwrap();
    project_dirs.data_dir().to_path_buf()
}

pub fn temp_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("", "", "squid").unwrap();
    project_dirs.cache_dir().to_path_buf()
}
