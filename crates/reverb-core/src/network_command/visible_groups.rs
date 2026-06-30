use std::{any::Any, collections::HashMap};

use crate::{failure::failure::{Failure, FailureType}, network_command::{ID::NetworkCommandID, group_info::GroupInfo, helpers::{NetworkCommand, QueryOrNotify}}};
use anyhow::anyhow;
use compact_str::CompactString;
use postcard::{from_bytes, to_slice};

#[derive(Debug, Clone)]
pub struct VisibleGroups {
    pub groups: HashMap<CompactString, GroupInfo>
}

impl NetworkCommand for VisibleGroups {
    fn number(&self) -> u8 {
        GroupInfo::ID
    }
    fn serialize(&self) -> Result<Vec<u8>, Failure> {
        let mut data = vec![];

        let mut buffer = [0u8; 512];
        let group_data = to_slice(&self.groups, &mut buffer)
            .map_err(|e| Failure::from((anyhow!("failed to serialize VisibleGroups: {e}"), FailureType::Warning)))?;
        data.extend_from_slice(group_data);

        Ok(data)
    }
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized {
        if data.len() < 2 {
            return Err(Failure::from((anyhow!("VisibleGroups Parsing Error: No Visible Groups"), FailureType::Warning)));
        }
        let groups: HashMap<CompactString, GroupInfo> = from_bytes(&data[1..])
            .map_err(|e| Failure::from((anyhow!("failed to parse VisibleGroups: {e}"), FailureType::Warning)))?;

        for (group, info) in &groups {
            println!("{group}, {}", info.id);
        }
        let visible_groups = VisibleGroups {
            groups
        };
        
        Ok(visible_groups)
    }

    fn query_or_notify(&self) -> QueryOrNotify {
        QueryOrNotify::Query
    }

    fn as_any(&self) -> &dyn Any { self }

}
