use std::{error::Error, fmt::Display, path::PathBuf};

use tokio::fs;

/// Contains boilerplate for managing packages

/// Searches for headers on the local machine
async fn retrieve_local_headers(
    local_repo_path: PathBuf,
    package_name: &str,
) -> Result<jellyfish::Package, RetrievePackageHeadersError> {
    let mut target_file = local_repo_path.join(package_name);
    target_file.set_extension("toml");
    // Make sure the target exists, and that it is
    if !target_file.exists() || !target_file.is_file() {
        return Err(RetrievePackageHeadersError::PackageNotFound(
            package_name.to_string(),
        ));
    }

    let package: jellyfish::Package =
        toml::from_str(fs::read_to_string(target_file).await.unwrap().as_str()).unwrap();

    Ok(package)
}

/// Updates the local repository, with a given other repository.
async fn update_repo(local_repo_path: PathBuf) -> Result<(), Box<dyn Error>> {
    // Locate git executable:
    #[cfg(windows)]
    let git_exec = PathBuf::from("git.exe");
    #[cfg(unix)]
    return Err("Unix not implemented yet");

    // TODO: added better ways of searching for git.exe
    if !git_exec.exists() {
        return Err(string_error::new_err("Failed to find git executable"));
    }

    println!("OMG it worked");

    Ok(())
}

enum RetrievePackageHeadersError {
    PackageNotFound(String),
    Unknown,
}

impl Display for RetrievePackageHeadersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "Error retrieving local package: {}",
                match self {
                    RetrievePackageHeadersError::PackageNotFound(package_name) =>
                        format!("{}", package_name),
                    RetrievePackageHeadersError::Unknown => todo!(),
                }
            )
            .as_str(),
        )
    }
}
