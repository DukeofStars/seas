use serde::{Deserialize, Serialize};

use crate::Package;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub url: String,
}

impl Repository {
    pub async fn connect(url: &str) -> Self {
        reqwest::get(url).await.expect(&format!("Failed to connect to remote repository at {}, it either does not exist, or there is no internet connection at the moment", url));
        Self {
            url: url.to_string(),
        }
    }

    pub async fn get_package(&self, name: &str) -> Result<Package, reqwest::Error> {
        let url = format!("{}/get/{}", self.url, name);
        let package: Package = reqwest::get(url).await?.json().await?;

        Ok(package)
    }
}
