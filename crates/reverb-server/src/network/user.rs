// all user specific logic not specific to networking

use std::sync::atomic::Ordering;
use compact_str::{CompactString, ToCompactString};

use reverb_core::{network::*, network_command::online_users::UserInfo};
use crate::{GROUPS, NEXT_USER_ID, ONLINE_USERS, OPEN_USERS, USERS, network::group::remove_group};

#[derive(Debug, Clone)]
pub struct User {
    id: u64,
    username: CompactString,
    show_online: bool,
    show_group: bool,
    open_to_echo: bool,
    group_info: Option<(u32, CompactString)>
}

pub fn register_new_user(packet: Packet) -> u64 {
    let user_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed); // wraps around when full overwriting existing users 
    let username = packet.username;
    let show_online = true;
    let show_group = true;
    let open_to_echo = true;
    let user = User::new(user_id, username, show_online, show_group, open_to_echo);

    add_user(user_id, user)
}

pub fn add_user(id: u64, user: User) -> u64 {
    USERS.insert(id, user.clone());
    if *user.online_status() {
        ONLINE_USERS.insert(id);
    }
    if *user.open_to_echo() {
        OPEN_USERS.insert(id);
    }

    id
}
pub fn remove_user(user_id: &u64) {
    if let Some((_, user)) = USERS.remove(user_id) {
        if let Some((group_id, _)) = user.group_info {
            let mut group_ref = GROUPS.get_mut(&group_id).unwrap();
            let group = group_ref.value_mut();
            group.users.remove(user_id);
            if group.users.is_empty() {
                remove_group(&group_id);
            }
        }
    }
    ONLINE_USERS.remove(user_id);
    OPEN_USERS.remove(user_id);
}

impl User {
    // general user info functions
    pub fn username(&self) -> &CompactString {
        &self.username
    }
    pub fn group(&self) -> &Option<(u32, CompactString)> {
        if self.show_group {
            return &self.group_info;
        }

        &None
    }
    pub fn open_to_echo(&self) -> &bool {
        &self.open_to_echo
    }
    pub fn user_info(&self) -> UserInfo {
        UserInfo {
            user_id: self.id,
            open_to_echo: self.open_to_echo,
            group_info: self.group().clone()
        }
    }

    // server functions
    pub fn online_status(&self) -> &bool {
        &self.show_online
    }
    pub fn new(id: u64, username: CompactString, show_online: bool, show_group: bool, open_to_echo: bool) -> Self {
        Self {
            id,
            username,
            show_online,
            show_group,
            open_to_echo,
            group_info: None
        }
    }
    pub fn set_echo_status(&mut self, open_to_echo: bool) {
        self.open_to_echo = open_to_echo;
    }
    pub fn set_online_status(&mut self, online_status: bool) {
        self.show_online = online_status;
    }
    pub fn update_group_info(&mut self, group_id: u32, group_name: CompactString) {
        self.group_info = Some((group_id, group_name));
    }
    pub fn remove_group(&mut self) {
        self.group_info = None;
    }
}
