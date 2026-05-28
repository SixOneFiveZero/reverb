use std::sync::atomic::Ordering;

use anyhow::anyhow;

use compact_str::ToCompactString;
use reverb_core::{failure::failure::{Failure, FailureType}, network::*, network_command::{ID::NetworkCommandID, default_command::DefaultCommand, get_online_users::GetOnlineUsers, online_users::UserInfo, set_echo_availability::SetEchoAvailability}};
use crate::{NEXT_USER_ID, SERVER_GROUP, SERVER_NAME, command_handling, network::connection::{User, add_user}};


pub fn handle_packet(packet: Packet, user_id: &u64) -> Result<Option<Packet>, Failure> {
    match packet.payload.number() {
        DefaultCommand::ID => {Ok(Some(Packet::new(SERVER_NAME, SERVER_GROUP, Box::new(DefaultCommand{}))?))},
        GetOnlineUsers::ID => {
            let outgoing_command = command_handling::handle_get_online_users(packet);
            Ok(Some(Packet {
                version: NETWORK_VERSION,
                username: SERVER_NAME.to_string(),
                group_id: SERVER_GROUP, 
                payload: outgoing_command
            }))
        },
        SetEchoAvailability::ID => {
            command_handling::handle_set_echo_availability(packet, user_id)?;
            Ok(None)
        },
        _ => {Err(Failure::from((anyhow!("packet handling error: command not found"), FailureType::Warning)))}
    }
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
