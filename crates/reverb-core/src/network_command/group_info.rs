use std::any::Any;

use compact_str::CompactString;
use postcard::{from_bytes, to_slice};
use serde::{Deserialize, Serialize};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    pub id: u32,
    pub group_name: CompactString,
    pub visible: bool,
    pub open: bool,
    pub users: Vec<CompactString>,
}

impl NetworkCommand for GroupInfo {
    fn number(&self) -> u8 {
        Self::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let mut buffer = [0u8; 512];
        let group_data = to_slice(&self, &mut buffer)
            .map_err(|e| Failure::from((anyhow!("failed to serialize CreateGroupResult: {e}"), FailureType::Warning)))?;

        let data = group_data.to_vec();
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let group_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to serialize CreateGroupResult: {e}"), FailureType::Warning)))?; 
        Ok(group_info)
    }
    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }
    fn as_any(&self) -> &dyn Any { self }
}
