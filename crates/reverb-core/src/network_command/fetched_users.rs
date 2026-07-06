// server responds with this when client sends FetchUsers command

use std::{any::Any, collections::HashSet};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;
use compact_str::CompactString;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchedUsers {
    pub users: HashSet<UserInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct UserInfo {
    pub user_id: u64,
    pub username: CompactString,
    pub open_to_echo: bool,
    pub group_info: Option<(u32, CompactString)> // id and name
}

impl NetworkCommand for FetchedUsers {
    fn number(&self) -> u8 {
        Self::ID
    }

    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let data = to_allocvec(&self)
            .map_err(|e| Failure::from((anyhow!("failed to serialize FetchedUsers: {e}"), FailureType::Warning)))?;
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let user_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to deserialize FetchedUsers: {e}"), FailureType::Warning)))?; 
        Ok(user_info)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl ToString for UserInfo {
    fn to_string(&self) -> String {
        format!("{} ({}) {} open to echo {}", 
            self.username, self.user_id, 
            if self.open_to_echo { "is" } else { "isn't" }, 
            match &self.group_info {
                Some((id, name)) => format!("in group {} ({})", name, id),
                None => "and not in a group".to_string()
            }
        )
    }
}
