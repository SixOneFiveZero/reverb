// server responds with this when client sends FetchUsers command

use std::{any::Any, collections::HashSet};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;
use compact_str::CompactString;
use postcard::{from_bytes, to_slice};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
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
        let mut data = vec![];

        let mut buffer = [0u8; 512];
        let user_data = to_slice(&self.users, &mut buffer)
            .map_err(|e| Failure::from((anyhow!("failed to serialize FetchedUsers: {e}"), FailureType::Warning)))?;
        data.extend_from_slice(user_data);

        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        if data.len() < 2 {
            return Err(Failure::from((anyhow!("FetchedUsers Parsing Error: No Online Users"), FailureType::Warning)));
        }
        let users: HashSet<UserInfo> = from_bytes(&data[1..])
            .map_err(|e| Failure::from((anyhow!("failed to parse FetchedUsers: {e}"), FailureType::Warning)))?;

        for user in &users {
            println!("{}, {}", user.username, user.user_id);
        }
        let fetched_users = FetchedUsers {
            users
        };
        
        Ok(fetched_users)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }

    fn as_any(&self) -> &dyn Any { self }

}
