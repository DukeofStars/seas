use std::{
    fs,
    sync::{Arc, Mutex},
};

use install::data_path;
use jellyfish::PackageVersion;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub mod install;
pub mod uninstall;

pub fn get_packages() -> Vec<PackageVersion> {
    let packages: Arc<Mutex<Vec<PackageVersion>>> = Arc::new(Mutex::new(Vec::new()));
    let folders = Vec::from_iter(data_path().read_dir().unwrap());
    folders.par_iter().for_each(|folder| {
        let folder = folder.as_ref().unwrap();
        if !folder.file_name().to_str().unwrap().starts_with(".") {
            // Try and read the package.toml file in the folder
            let package_toml = folder.path().join("package.toml");
            let package: PackageVersion = toml::from_str(
                &fs::read_to_string(package_toml)
                    .expect("Package does not contain a valid package.toml file"),
            )
            .expect("Invalid package.toml file");

            let state_ref = Arc::clone(&packages);
            let mut state = state_ref.lock().unwrap();
            state.push(package);
        }
    });
    let packages = packages.lock().unwrap();
    packages.to_vec()
}
