use std::path::Path;
use anyhow::anyhow;

use crate::{config::{config::{Config, config}, data::StartupData}, internal::internal::Internal};
use reverb_core::failure::failure::{Failure, FailureType};

pub fn startup() -> Result<Internal, Failure> {
    println!("Starting up... ");

    Config::load()?;

    let config = config()?;
    let data_folder = Path::new(&config.data_folder);
    
    // if exists, use it if not create and use
    println!("Loading startup data... ");
    let mut startup_data ;
    if data_folder.join("startup.toml").exists() {
        startup_data = toml::from_str(
            &std::fs::read_to_string(data_folder.join("startup.toml"))
            .map_err(|e| Failure::from((e.into(), FailureType::Fatal)))?
        ).map_err(|e| Failure::from((e.into(), FailureType::Fatal)))?;
    } else {
        std::fs::create_dir_all(data_folder).map_err(|e| Failure::from((e.into(), "create_dir_all failed", FailureType::Fatal)))?;
        startup_data = StartupData::new_default().map_err(|_| Failure::from((anyhow!("StartupData::new_default() Failed"), FailureType::Fatal)))?;
        println!("First run?: \n Default startup data created in {} \n no need to restart, continuing automatically\n enjoy REVERB!", data_folder.display());
    }

    if !startup_data.last_shutdown_clean {
        println!("Warning: Last shutdown was not clean, data may be corrupted, lost or incorrect. \n Attempting to continue... ");
    }

    startup_data.last_shutdown_clean = false;
    startup_data.save()?;
        

    Ok(Internal::new(startup_data.queue)?)
}

pub fn shutdown (internal: &Internal) -> Result<(), Failure> {
    println!("Shutting down... ");

    println!("Saving startup data... ");
    StartupData {
        queue: internal.queue_get().clone(),
        last_shutdown_clean: true,
    }.save()?;

    println!("Shutting down internal... ");
    internal.shutdown()?;

    Ok(())
}
