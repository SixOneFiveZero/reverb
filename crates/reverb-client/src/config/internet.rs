
use super::config::CONFIG_FOLDER;
use anyhow::anyhow;
use arc_swap::ArcSwap;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use reverb_core::failure::failure::{Failure, FailureType};

static SERVER_CONFIG_PATH: &str = "server_config.toml";
static SERVER_CONFIG: OnceCell<ArcSwap<ServerConfig>> = OnceCell::new();

/// ServerConfig struct represents the server config file.
/// do not edit data directly, use the update_server_config function to update
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_address: String,
    pub server_name: String,
    pub server_cert_path: String,
    pub echo_avaliable: bool,
}

impl ServerConfig {
    fn new(server_address: &str, server_name: &str, server_cert_path: &str, echo_avaliable: bool) -> Result<ServerConfig, Failure> {
        let server_config = ServerConfig {
            server_address: server_address.to_string(),
            server_name: server_name.to_string(),
            server_cert_path: server_cert_path.to_string(),
            echo_avaliable,
        };
        server_config.save()?;
        Ok(server_config)
    }

    fn save(&self) -> Result<(), Failure> {
        match std::fs::write(
            format!("{}server_config.toml", CONFIG_FOLDER),
            toml::to_string(self).map_err(|e| Failure::from((e.into(), FailureType::Warning)))?,
        ) {
            Err(e) => Err(Failure::from((e.into(), FailureType::Warning))),
            Ok(_) => Ok(()),
        }
    }

    /// loads the server config from the config folder into the static variable, if it doesn't exist it will return an error
    fn load() -> Result<(), Failure> {
        let server_config = toml::from_str::<ServerConfig>(&std::fs::read_to_string(format!("{}{}", CONFIG_FOLDER, SERVER_CONFIG_PATH))
            .map_err(|e| Failure::from((e.into(), "Failed to read server config, to add a server please run the server setup command", FailureType::Warning)))?)
            .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;
        SERVER_CONFIG.set(ArcSwap::new(std::sync::Arc::new(server_config)))
            .map_err(|_| Failure::from((anyhow!("Failed to set server config"), FailureType::Fatal)))?;
        Ok(())
    }
}

/// for reading the server config
/// if the server config is not loaded it will attempt to load it, if it fails to load it will return an error 
pub fn server_config() -> Result<arc_swap::Guard<std::sync::Arc<ServerConfig>>, Failure> {
    if SERVER_CONFIG.get().is_none() {
        ServerConfig::load()?;
    }
    Ok(SERVER_CONFIG.get().unwrap().load())
}

/// for updating the server config, only updates the fields that are Some, if a field is None it will keep the old value
/// to create a new server config, set all fields to Some and call this function
pub fn update_server_config(server_address: Option<&str>, server_name: Option<&str>, server_cert_path: Option<&str>, echo_avaliable: Option<bool>) -> Result<(), Failure> {
    if SERVER_CONFIG.get().is_none() {
        if let Err(e) = ServerConfig::load() {
            // if the server config fails to load, all fields must be Some to create a new server config, if any field is None it will return an error
            if server_address.is_none() || server_name.is_none() || server_cert_path.is_none() || echo_avaliable.is_none() {
                return Err(e);
            } else {
                let server_config = ServerConfig::new(server_address.unwrap(), server_name.unwrap(), server_cert_path.unwrap(), echo_avaliable.unwrap())?;
                SERVER_CONFIG.set(ArcSwap::new(std::sync::Arc::new(server_config)))
                    .map_err(|_| Failure::from((anyhow!("Failed to set server config"), FailureType::Fatal)))?;
                return Ok(());
            }
        }
    }

    // if the server config is loaded, update the fields that are Some and keep the old value for the fields that are None
    let cfg = SERVER_CONFIG.get().unwrap();
    cfg.rcu(|cfg| {
        let new_server_config = ServerConfig {
            server_address: server_address.unwrap_or(&cfg.server_address).to_string(),
            server_name: server_name.unwrap_or(&cfg.server_name).to_string(),
            server_cert_path: server_cert_path.unwrap_or(&cfg.server_cert_path).to_string(),
            echo_avaliable: echo_avaliable.unwrap_or(cfg.echo_avaliable),
        };
        std::sync::Arc::new(new_server_config)
    });
    server_config()?.save()?;
    Ok(())
}