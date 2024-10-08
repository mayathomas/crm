use anyhow::{bail, Result};
use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub port: u16,
    pub sender_email: String,
    pub metadata: String,
    pub user_stats: String,
    pub notification: String,
    pub tls: Option<TlsConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub pk: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from ./app.yml, or /etc/config/app.yml, or from env CHAT_CONFIG
        let ret = match (
            File::open("crm.yml"),
            File::open("/etc/config/crm.yaml"),
            std::env::var("CRM_CONFIG"),
        ) {
            (Ok(file), _, _) => serde_yaml::from_reader(file),
            (_, Ok(file), _) => serde_yaml::from_reader(file),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Failed to load config file"), // _ => return Err(anyhow::anyhow!("Failed to load config file")),
        };
        Ok(ret?)
    }
}
