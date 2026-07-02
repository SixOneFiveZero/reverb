// all group specific logic

use std::collections::BTreeMap;

use compact_str::CompactString;
use reverb_core::network_command::group_info::GroupInfo;

use crate::{GROUPS, OPEN_GROUPS, USERS, VISIBLE_GROUPS};

#[derive(Debug, Clone)]
pub struct Group {
    pub group_name: CompactString,
    pub id: u32,
    pub visible: bool,
    pub open: bool,
    pub users: BTreeMap<u64, CompactString>,
    pub host: u64
}

pub fn add_group(id: u32, group: Group) -> u32 {
    GROUPS.insert(id, group.clone());
    if group.visible {
        VISIBLE_GROUPS.insert(id);
    }
    if group.open {
        OPEN_GROUPS.insert(id);
    }

    id
}
pub fn remove_group(id: &u32) {
    if let Some(group_ref) = GROUPS.get(id) {
        for user_id in group_ref.users.keys() {
            USERS.get_mut(user_id).as_mut().unwrap().remove_group();
        }
    }
    GROUPS.remove(id);
    VISIBLE_GROUPS.remove(id);
    OPEN_GROUPS.remove(id);
}

impl Group {
    pub fn get_info(&self) -> GroupInfo {
        GroupInfo {
            id: self.id,
            name: self.group_name.clone(),
            visible: self.visible,
            open: self.open,
            users: self.users.clone(),
            host: self.host
        }
    }
}
