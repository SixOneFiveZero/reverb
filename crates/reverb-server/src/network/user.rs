// all user specific logic not specific to networking

use std::sync::atomic::Ordering;
use compact_str::{CompactString, ToCompactString};

use reverb_core::{network::*, network_command::online_users::UserInfo};
use crate::{GROUPS, NEXT_USER_ID, ONLINE_USERS, OPEN_USERS, USERS, network::group::remove_group};

#[derive(Debug, Clone)]
pub struct User {
    username: CompactString,
    user_info: UserInfo,
    show_online: bool,
    is_group_visible: bool,
}

pub fn register_new_user(packet: Packet) -> u64 {
    let user_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed); // wraps around when full overwriting existing users 
    let username = packet.username.to_compact_string();
    let user_info = UserInfo {
        user_id,
        group_id: 0,
        open_to_echo: false,
    };
    let online_status = true;
    let user = User::new(username, user_info, online_status);

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
        let group_id = user.user_info.group_id;
        let mut group_ref = GROUPS.get_mut(&group_id).unwrap();
        let group = group_ref.value_mut();
        group.users.remove(user_id);
        if group.users.is_empty() {
            remove_group(&group_id);
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
    pub fn group_id(&self) -> &u32 {
        if self.is_group_visible {
            &self.user_info.group_id
        } else {
            &0
        }
    }
    pub fn open_to_echo(&self) -> &bool {
        &self.user_info.open_to_echo
    }
    pub fn user_info(&self, id: u64) -> UserInfo {
        UserInfo {
            user_id: id,
            group_id: *self.group_id(),
            open_to_echo: *self.open_to_echo(),
        }
    }

    // server functions
    pub fn online_status(&self) -> &bool {
        &self.show_online
    }
    pub fn new(username: CompactString, user_info: UserInfo, show_online: bool) -> Self {
        Self {
            username,
            user_info,
            show_online,
            is_group_visible: false,
        }
    }
    pub fn set_echo_status(&mut self, open_to_echo: bool) {
        self.user_info.open_to_echo = open_to_echo;
    }
    pub fn set_online_status(&mut self, online_status: bool) {
        self.show_online = online_status;
    }
}
