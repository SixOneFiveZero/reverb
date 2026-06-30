// command to request specific data from the server

use std::any::Any;

use postcard::{from_bytes, to_slice};
use serde::{Deserialize, Serialize};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fetch {
    Users(UserFilter),
    Groups(GroupFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilter {
    pub open_to_echo: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupFilter {
    pub open: bool
}


impl NetworkCommand for Fetch {
    fn number(&self) -> u8 {
        Self::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let mut buffer = [0u8; 512];
        let group_data = to_slice(&self, &mut buffer)
            .map_err(|e| Failure::from((anyhow!("failed to serialize Fetch: {e}"), FailureType::Warning)))?;

        let data = group_data.to_vec();
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let group_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to deserialize Fetch: {e}"), FailureType::Warning)))?; 
        Ok(group_info)
    }
    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }
    fn as_any(&self) -> &dyn Any { self }
}
