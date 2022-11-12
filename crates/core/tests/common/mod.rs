use std::{error::Error, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub uid: String,
    pub phone_num: String,
    pub password: String,
    pub school_code: String,
    pub session_token: String,
    pub device_id: String,
    pub campus_token: String,
}

impl Config {
    #[allow(dead_code)]
    pub async fn parse(path: &Path) -> Result<Self, Box<dyn Error>> {
        let reader = tokio::fs::File::open(path).await?;

        let config: Config = serde_yaml::from_reader(reader.into_std().await)?;

        Ok(config)
    }

    #[allow(dead_code)]
    pub fn parse_blocking(path: &Path) -> Result<Self, Box<dyn Error>> {
        let reader = std::fs::File::open(path)?;

        let config: Config = serde_yaml::from_reader(reader)?;

        Ok(config)
    }
}
