// placeholder failure command,
// TODO: improve this

use std::any::Any;

use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

// TODO better error handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkFailure {
    InvalidCommandId(u8),
    JoinGroup(String)
}

impl NetworkCommand for NetworkFailure {
    fn number(&self) -> u8 {
        Self::ID
    }

    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let data = to_allocvec(&self)
            .map_err(|e| Failure::from((anyhow!("failed to serialize NetworkFailure: {e}"), FailureType::Warning)))?;
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let group_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to deserialize NetworkFailure: {e}"), FailureType::Warning)))?; 
        Ok(group_info)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl ToString for NetworkFailure {
    fn to_string(&self) -> String {
        format!("NetworkFailure: {:#?}", self)
    }
}