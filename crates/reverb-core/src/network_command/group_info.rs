// command sent by server with all the information about a group,
// sent upon group creation, joining a group or as part of FetchedGroups command

use std::{any::Any, collections::BTreeMap};

use compact_str::CompactString;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct GroupInfo {
    pub id: u32,
    pub name: CompactString,
    pub visible: bool,
    pub open: bool,
    pub users: BTreeMap<u64, CompactString>,
    pub host: u64
}

impl NetworkCommand for GroupInfo {
    fn number(&self) -> u8 {
        Self::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let data = to_allocvec(&self)
            .map_err(|e| Failure::from((anyhow!("failed to serialize GroupInfo: {e}"), FailureType::Warning)))?;
        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        let group_info: Self = from_bytes(&data)
            .map_err(|e| Failure::from((anyhow!("failed to deserialize GroupInfo: {e}"), FailureType::Warning)))?; 
        Ok(group_info)
    }
    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Notify
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl ToString for GroupInfo {
    fn to_string(&self) -> String {
        let mut group_string = format!("{} ({}) {} open to echo, users:\n", self.name, self.id, if self.open {"is"} else {"isn't"});
        for user in self.users.clone() {
            group_string.push_str(&format!("  {}", user.1));
            if user.0 == self.host {
                group_string.push_str(" (host)");
            }
            group_string.push('\n');
        }
        group_string
    }
}