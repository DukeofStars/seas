use std::{
    fs,
    io::{self, Write},
    os,
    path::PathBuf,
};

use colored::Colorize;
use compression::prelude::{DecodeExt, GZipDecoder};
use directories::ProjectDirs;
use jellyfish::{request::Repository, Package, PackageVersion};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use trauma::{download::Download, downloader::DownloaderBuilder};

#[derive(Default, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Installer {
    pub file_path: PathBuf,
    pub relative_path: PathBuf,
    pub download_path: String,
    pub bin_folder: PathBuf,
    pub stage: InstallStages,
    pub repo: Repository,
    pub package: Package,
    pub package_version: PackageVersion,
    pub uncompressed: Vec<u8>,
}

impl Installer {
    pub async fn set_stage(&mut self, stage: InstallStages) {
        self.stage = stage;

        let path = temp_path().join("Continue.toml");

        let toml = toml::Value::try_from(&self).unwrap();

        fs::write(path, toml.to_string()).unwrap();
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
                name: Default::default(),
                version: "0.0.0".to_string(),
                required: Default::default(),
                dependencies: Default::default(),
                flavor: "vanilla".to_string(),
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
        me.set_stage(InstallStages::ConnectToRepository).await;
        // Connect to the repository
        let repository = "http://localhost:8000/jellyfish/"; // TODO: load this from config file and allow for multiple repositories
        me.repo = Repository::connect(repository).await;

        me
    }

    pub async fn find_package(&mut self, name: &str) -> &mut Self {
        self.set_stage(InstallStages::FindPackage).await;
        if data_path().join(&name).exists() {
            fs::remove_dir_all(data_path().join(&name)).unwrap();
        }
        // Get pacakge from server
        self.package = self
            .repo
            .get_package(name)
            .await
            .expect("Failed to find requested package from the remote");

        self
    }

    pub async fn get_package_version(&mut self, version: Option<String>) -> &mut Self {
        self.set_stage(InstallStages::GetPackageVersion).await;
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
        self.set_stage(InstallStages::PreDownload).await;
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
        self.set_stage(InstallStages::Download).await;
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
        self.set_stage(InstallStages::Extract).await;
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
        self.set_stage(InstallStages::Unwrap).await;
        // Unwrap tar ball
        let mut tar_ball = tar::Archive::new(self.uncompressed.as_slice());
        tar_ball
            .unpack(data_path().join(&self.package.name).as_path())
            .unwrap();

        self
    }

    pub async fn pre_install(&mut self) -> &mut Self {
        self.set_stage(InstallStages::PreInstall).await;
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
        self.set_stage(InstallStages::Install).await;
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
            let target = data_path().join(".bin").join(entry.file_name());
            let target_root = target.parent().unwrap();
            if !target_root.exists() {
                fs::create_dir_all(target_root).unwrap();
            }
            // If the target already exists, ask the user if they would like to skip this file or replace it.
            // We use fs::read_link instead of PathBuf::exists because PathBuf::exists traverse symbolic links, and we don't want that.
            if fs::read_link(&target).is_ok() {
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
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
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
        self.set_stage(InstallStages::Cleanup).await;
        // Remove the temp file.
        fs::remove_file(&self.file_path).unwrap();
        // Remove the continue file.
        fs::remove_file(temp_path().join("Continue.toml")).unwrap();

        // Write package.toml if it doesn't exist.
        let package_toml = data_path().join(&self.package.name).join("package.toml");
        if !package_toml.exists() {
            let mut file = fs::File::create(package_toml).unwrap();
            file.write_all(
                toml::to_string_pretty(&self.package_version)
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        }

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
