use crate::network_command::{create_new_group::CreateNewGroup, default_command::DefaultCommand, echo::Echo, failure::NetworkFailure, fetch_groups::FetchGroups, fetch_users::FetchUsers, fetched_groups::FetchedGroups, fetched_users::FetchedUsers, group_info::GroupInfo, join_group::JoinGroup, set_echo_availability::SetEchoAvailability, set_online_status::SetOnlineStatus, skip::Skip, user_data::UserData};

pub trait NetworkCommandID {
    const ID: u8;
}

impl NetworkCommandID for DefaultCommand {
    const ID: u8 = 0;
}
impl NetworkCommandID for Skip {
    const ID: u8 = 1;
}
impl NetworkCommandID for Echo {
    const ID: u8 = 2;
}
impl NetworkCommandID for FetchedUsers {
    const ID: u8 = 3;
}
impl NetworkCommandID for UserData {
    const ID: u8 = 4;
}
impl NetworkCommandID for FetchUsers {
    const ID: u8 = 5;
}
impl NetworkCommandID for SetEchoAvailability {
    const ID: u8 = 6;
}
impl NetworkCommandID for SetOnlineStatus {
    const ID: u8 = 7;
}
impl NetworkCommandID for CreateNewGroup {
    const ID: u8 = 8;
}
impl NetworkCommandID for GroupInfo {
    const ID: u8 = 9;
}
impl NetworkCommandID for JoinGroup {
    const ID: u8 = 10;
}
impl NetworkCommandID for NetworkFailure {
    const ID: u8 = 11;
}
impl NetworkCommandID for FetchGroups {
    const ID: u8 = 12;
}
impl NetworkCommandID for FetchedGroups {
    const ID: u8 = 13;
}
