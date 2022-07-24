use crate::Package;

pub struct Repository {
    pub url: String,
}

impl Repository {
    pub fn connect(url: &str) -> Self {
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
