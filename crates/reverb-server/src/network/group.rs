// all group specific logic

use std::{collections::HashSet};
use anyhow::anyhow;
use compact_str::CompactString;
use quinn::{Connection, Incoming, RecvStream};

use reverb_core::{failure::failure::{Failure, FailureType}, network::*, network_command::online_users::UserInfo};
use crate::{GROUPS, ONLINE_USERS, OPEN_GROUPS, OPEN_USERS, USERS, VISIBLE_GROUPS, network::packet_handling::handle_packet};

#[derive(Debug, Clone)]
pub struct Group {
    pub group_name: CompactString,
    pub users: HashSet<u64>,
    pub host: u64,
    pub is_group_open: bool,
    pub is_group_visible: bool,
}

pub fn add_group(id: u32, group: Group) -> u32 {
    GROUPS.insert(id, group.clone());
    if group.is_group_visible {
        VISIBLE_GROUPS.insert(id);
    }
    if group.is_group_open {
        OPEN_GROUPS.insert(id);
    }

    id
}
pub fn remove_group(id: &u32) {
    GROUPS.remove(id);
    VISIBLE_GROUPS.remove(id);
    OPEN_GROUPS.remove(id);
}
