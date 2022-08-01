use std::fs::{self, DirEntry};

use jellyfish::PackageVersion;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{data_path, get_packages};

#[derive(Debug)]
pub struct Uninstaller {
    package_version: PackageVersion,
}

impl Uninstaller {
    pub async fn find_package(name: &str) -> Result<Self, String> {
        let packages = get_packages();
        packages
            .iter()
            .find(|package| package.name == name)
            .map(|package| Self {
                package_version: package.clone(),
            })
            .ok_or(format!("Package {} not found", name))
    }

    pub async fn unlink(&self) {
        let bin_folder = data_path().join(&self.package_version.name).join("bin");
        let bin_files: Vec<Result<DirEntry, std::io::Error>> =
            bin_folder.read_dir().unwrap().collect();
        bin_files.par_iter().for_each(|file| {
            let file = file.as_ref().unwrap();
            let target = data_path().join(".bin").join(file.file_name());
            if target.is_dir() {
                fs::remove_dir(&target).unwrap();
            } else if target.is_file() {
                fs::remove_file(&target).unwrap();
            }
        });
    }

    pub async fn delete(&self) {
        fs::remove_dir_all(data_path().join(&self.package_version.name)).unwrap();
    }
}
