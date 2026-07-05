// send to server to request groups matching specified flags,
// server returns FetchedGroups

use std::any::Any;
use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct FetchGroups {
    pub open: Option<bool>,
}

impl NetworkCommand for FetchGroups {
    fn number(&self) -> u8 {
        Self::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let group_data = match self.open {
            Some(open) => [open as u8 + 1], // +1 so we can use 0 to represent None value of option as 0
            None => [0],
        };

        Ok(group_data.to_vec())
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let group_data = match data[0] {
            0 => None,
            1 => Some(false),
            2 => Some(true),
            _ => { return Err(Failure::from((anyhow!("Failed to parse FetchUsers: Invalid open_to_echo value"), FailureType::Warning))); }
        };

        Ok(FetchGroups { open: group_data })
    }
    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Query
    }
    fn as_any(&self) -> &dyn Any { self }
}
