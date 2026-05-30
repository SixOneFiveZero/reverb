use crate::CONFIG;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use reverb_core::failure::failure::{Failure, FailureType};

pub static SERVER_CONFIG_PATH: &str = "server_config.toml";


#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_address: String,
    pub server_name: String,
    pub server_cert_path: String,
    pub echo_avaliable: bool,
}

impl ServerConfig {
    pub fn new(server_address: &str, server_name: &str, server_cert_path: &str, echo_avaliable: bool) -> Result<ServerConfig, Failure> {
        let server_config = ServerConfig {
            server_address: server_address.to_string(),
            server_name: server_name.to_string(),
            server_cert_path: server_cert_path.to_string(),
            echo_avaliable,
        };
        server_config.save()?;
        Ok(server_config)
    }

    pub fn save(&self) -> Result<(), Failure> {
        match std::fs::create_dir_all(CONFIG.get().ok_or(Failure::from((anyhow!("Config folder not found"), FailureType::Fatal)))?.data_folder.clone()) {
            Err(e) => return Err(Failure::from((e.into(), FailureType::Warning))),
            Ok(_) => {},
        }
        match std::fs::write(
            format!("{}server_config.toml", CONFIG.get().ok_or(Failure::from((anyhow!("Config folder not found"), FailureType::Fatal)))?.data_folder.clone()),
            toml::to_string(self).map_err(|e| Failure::from((e.into(), FailureType::Warning)))?,
        ) {
            Err(e) => Err(Failure::from((e.into(), FailureType::Warning))),
            Ok(_) => Ok(()),
        }
    }

    pub fn load() -> Result<ServerConfig, Failure> {
        let data_folder = crate::DATA_FOLDER.get().ok_or(Failure::from((anyhow!("Data folder not found"), FailureType::Fatal)))?.clone();
        let server_config = toml::from_str::<ServerConfig>(&std::fs::read_to_string(format!("{}{}", data_folder, SERVER_CONFIG_PATH))
            .map_err(|e| Failure::from((e.into(), "Failed to read server config, to add a server please run the server setup command", FailureType::Warning)))?)
            .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;
        Ok(server_config)
    }
}
