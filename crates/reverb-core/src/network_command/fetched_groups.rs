// server responds with this when client sends FetchGroups command

use std::{any::Any, collections::HashSet};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, group_info::GroupInfo, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;
use postcard::{from_bytes, to_slice};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchedGroups {
    pub groups: HashSet<GroupInfo>
}

impl NetworkCommand for FetchedGroups {
    fn number(&self) -> u8 {
        GroupInfo::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let mut data = vec![];

        let mut buffer = [0u8; 512];
        let group_data = to_slice(&self.groups, &mut buffer)
            .map_err(|e| Failure::from((anyhow!("failed to serialize FetchedGroups: {e}"), FailureType::Warning)))?;
        data.extend_from_slice(group_data);

        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        if data.len() < 2 {
            return Err(Failure::from((anyhow!("FetchedGroups Parsing Error: No Groups Fetched"), FailureType::Warning)));
        }
        let groups: HashSet<GroupInfo> = from_bytes(&data[1..])
            .map_err(|e| Failure::from((anyhow!("failed to parse VisibleGroups: {e}"), FailureType::Warning)))?;

        for group in &groups {
            println!("{}, {}", group.name, group.id);
        }
        let visible_groups = FetchedGroups {
            groups
        };
        
        Ok(visible_groups)
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
            let mut group_string = format!("{} ({}) {} open to echo, users:\n", group.name, group.id, if group.open {"is"} else {"isn't"});
            for user in group.users.clone() {
                group_string.push_str(&format!("  {}", user.1));
                if user.0 == group.host {
                    group_string.push_str(" (host)");
                }
                group_string.push('\n');
            }
            group_strings.push(group_string);
        }
        group_strings.join("\n")
    }
}