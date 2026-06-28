use crate::network_command::{create_new_group::CreateNewGroup, default_command::DefaultCommand, echo::Echo, get_online_users::GetOnlineUsers, group_info::GroupInfo, group_invitation::GroupInvitation, invite_to_group::InviteToGroup, online_users::OnlineUsers, set_echo_availability::SetEchoAvailability, set_online_status::SetOnlineStatus, skip::Skip, user_data::UserData};

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
impl NetworkCommandID for OnlineUsers {
    const ID: u8 = 3;
}
impl NetworkCommandID for UserData {
    const ID: u8 = 4;
}
impl NetworkCommandID for GetOnlineUsers {
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
impl NetworkCommandID for GroupInvitation {
    const ID: u8 = 10;
}
impl NetworkCommandID for InviteToGroup {
    const ID: u8 = 11;
}
