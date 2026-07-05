// send to server to request users matching specified flags,
// server returns FetchedUsers

use std::any::Any;

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct FetchUsers {
    pub open_to_echo: Option<bool>,
}

impl NetworkCommand for FetchUsers {
    fn number(&self) -> u8 {
        Self::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let user_data = match self.open_to_echo {
            Some(open) => [open as u8 + 1], // +1 so we can use 0 to represent None value of option as 0
            None => [0],
        };

        Ok(user_data.to_vec())
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let user_data = match data[0] {
            0 => None,
            1 => Some(false),
            2 => Some(true),
            _ => { return Err(Failure::from((anyhow!("Failed to parse FetchUsers: Invalid open_to_echo value"), FailureType::Warning))); }
        };

        Ok(FetchUsers { open_to_echo: user_data })
    }
    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Query
    }
    fn as_any(&self) -> &dyn Any { self }
}
