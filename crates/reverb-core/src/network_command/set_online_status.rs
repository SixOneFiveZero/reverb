use std::any::Any;

use crate::{network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}, failure::failure::{Failure, FailureType}};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct SetOnlineStatus(pub bool);

impl NetworkCommand for SetOnlineStatus {
    fn number(&self) -> u8 {
        SetOnlineStatus::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        Ok(vec![self.0 as u8])
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        if data.len() != 1 {
            return Err(Failure::from((anyhow!("invalid data length for SetOnlineStatus"), FailureType::Warning)));
        }
        let availability = match data[0] {
            0 => false,
            1 => true,
            _ => return Err(Failure::from((anyhow!("invalid value for SetOnlineStatus"), FailureType::Warning)))
        };
        Ok(SetOnlineStatus(availability))
    }
    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }
    fn as_any(&self) -> &dyn Any { self }
}
