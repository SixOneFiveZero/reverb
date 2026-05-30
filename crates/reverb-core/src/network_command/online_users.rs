use std::{any::Any, collections::HashMap};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}, online_users}};
use anyhow::anyhow;
use compact_str::CompactString;
use postcard::{from_bytes, to_slice};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OnlineUsers {
    pub users: HashMap<CompactString, UserInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: u64,
    pub group_id: u32,
    pub open_to_echo: bool,
}

impl NetworkCommand for OnlineUsers {
    fn number(&self) -> u8 {
        OnlineUsers::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let mut data = vec![];

        let mut buffer = [0u8; 512];
        let user_data = to_slice(&self.users, &mut buffer)
            .map_err(|e| Failure::from((anyhow!("failed to serialize OnlineUsers: {e}"), FailureType::Warning)))?;
        data.extend_from_slice(user_data);

        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        if data.len() < 2 {
            return Err(Failure::from((anyhow!("OnlineUsers Parsing Error: No Online Users"), FailureType::Warning)));
        }
        let users: HashMap<CompactString, UserInfo> = from_bytes(&data[1..])
            .map_err(|e| Failure::from((anyhow!("failed to parse OnlineUsers: {e}"), FailureType::Warning)))?;

        for (user, info) in &users {
            let a = info.user_id;
            println!("{user}, {a}");
        }
        let online_users = OnlineUsers {
            users
        };
        
        Ok(online_users)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Query
    }

    fn as_any(&self) -> &dyn Any { self }

}
