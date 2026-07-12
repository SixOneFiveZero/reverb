// server responds with this when client sends FetchGroups command

use std::{any::Any, collections::HashSet};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, group_info::GroupInfo, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchedGroups {
    pub groups: HashSet<GroupInfo>
}

impl NetworkCommand for FetchedGroups {
    fn number(&self) -> u8 {
        Self::ID
    }
        
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let data = to_allocvec(&self)
            .map_err(|e| Failure::from((anyhow!("failed to serialize FetchedGroups: {e}"), FailureType::Warning)))?;
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let group_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to deserialize FetchedGroups: {e}"), FailureType::Warning)))?; 
        Ok(group_info)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl ToString for FetchedGroups {
    fn to_string(&self) -> String {
        let mut group_strings = vec![];
        for group in &self.groups {
            group_strings.push(group.to_string());
        }
        group_strings.join("\n")
    }
}
