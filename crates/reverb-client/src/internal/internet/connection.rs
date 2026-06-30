use std::sync::mpsc;
use anyhow::{Result, anyhow};

use crate::{config::{config::config, internet::server_config}, internal::internet::communicator};
use reverb_core::{network_command::helpers::NetworkCommand, failure::failure::{Failure, FailureType}, network::*};



#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    Connected(mpsc::Sender<Packet>),
    Connecting,
    NotConnected,
}

pub struct InternetClient {
    connection_status: ConnectionStatus,
    group_id: u32
}

impl InternetClient {
    pub fn new() -> Self {
        let _ = rustls::crypto::ring::default_provider().install_default();
        InternetClient { 
            connection_status: ConnectionStatus::NotConnected,
            group_id: 0
        }
    }

    pub fn connect(&mut self) -> Result<(), Failure> {
        match self.connection_status {
            ConnectionStatus::Connected(_) => {
                Err(Failure::from((anyhow!("Already connected to server"), FailureType::Warning)))
            },
            ConnectionStatus::Connecting => {
                Err(Failure::from((anyhow!("Already connecting to server"), FailureType::Warning)))
            },
            ConnectionStatus::NotConnected => {
                self.connection_status = ConnectionStatus::Connecting;

                let server_config = server_config()?;
                println!("Attempting to connect to server at {} with name {} and certificate path {}", server_config.server_address, server_config.server_name, server_config.server_cert_path);
                communicator::start_communicator_thread();
                Ok(())
            },
        }
    }

    pub fn send_message(&mut self, command: Box<dyn NetworkCommand + Send + Sync>) -> Result<(), Failure> {
        println!("Attempting to send message to server: ");
        let packet = Packet::new(
            config()?.username.as_str(),
            self.group_id,
            command
        )?;
        match &mut self.connection_status {
            ConnectionStatus::Connected(sender) => {sender.clone().send(packet).map_err(|e| Failure::from((e.into(), FailureType::Warning)))},
            ConnectionStatus::Connecting => Err(Failure::from((anyhow!("Currently connecting to server, cannot send message"), FailureType::Warning))),
            ConnectionStatus::NotConnected => Err(Failure::from((anyhow!("Not connected to server, cannot send message"), FailureType::Warning))),
        }
    }

    pub fn update_connection(&mut self, connection_status: ConnectionStatus) {
        self.connection_status = connection_status;
    }
}

