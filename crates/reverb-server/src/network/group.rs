// all group specific logic

use std::{collections::HashSet};
use compact_str::CompactString;

use crate::{GROUPS, OPEN_GROUPS, VISIBLE_GROUPS};

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
