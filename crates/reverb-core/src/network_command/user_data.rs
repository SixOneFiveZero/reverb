// data sent from client in initial handshake

use std::any::Any;

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct UserData {
    pub echo_avaliable: bool,
}

impl NetworkCommand for UserData {
    fn number(&self) -> u8 {
        UserData::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        Ok(vec![self.echo_avaliable as u8])
    }
    fn parse(_data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let echo_avaliable = match _data[1] {
            0 => false,
            1 => true,
            _ => return Err(Failure::from((anyhow!("invalid value for UserData"), FailureType::Warning)))
        };
        Ok(UserData { echo_avaliable })
    }
    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }
    fn as_any(&self) -> &dyn Any { self }
}
