// client sends command to request the server to create a new group, 
// server responds with GroupInfo

use std::any::Any;

use compact_str::CompactString;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewGroup {
    pub group_name: CompactString,
    pub open: bool,
    pub visible: bool,
    pub invited_users: Vec<u64>
}

impl NetworkCommand for CreateNewGroup {
    fn number(&self) -> u8 {
        CreateNewGroup::ID
    }

    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let data = to_allocvec(&self)
            .map_err(|e| Failure::from((anyhow!("failed to serialize CreateNewGroup: {e}"), FailureType::Warning)))?;
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let group_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to deserialize CreateNewGroup: {e}"), FailureType::Warning)))?; 
        Ok(group_info)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Query
    }
    fn as_any(&self) -> &dyn Any { self }
}
