// command to join group with valid id, 
// server responds with GroupInfo if successful or NetworkFailure if not

use std::any::Any;

use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGroup {
    pub group_id: u32,
}

impl NetworkCommand for JoinGroup {
    fn number(&self) -> u8 {
        Self::ID
    }

    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let data = to_allocvec(&self)
            .map_err(|e| Failure::from((anyhow!("failed to serialize JoinGroup: {e}"), FailureType::Warning)))?;
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let user_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to deserialize JoinGroup: {e}"), FailureType::Warning)))?; 
        Ok(user_info)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Query
    }

    fn as_any(&self) -> &dyn Any { self }
}

