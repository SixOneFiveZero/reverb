// identify the command based on its ID, pass to handler function

use anyhow::anyhow;

use compact_str::ToCompactString;
use reverb_core::{failure::failure::{Failure, FailureType}, network::*, network_command::{ID::NetworkCommandID, create_new_group::CreateNewGroup, default_command::DefaultCommand, get_online_users::GetOnlineUsers, get_visible_groups::GetVisibleGroups, helpers::NetworkCommand, join_group::JoinGroup, set_echo_availability::SetEchoAvailability, set_online_status::SetOnlineStatus}};
use crate::{SERVER_GROUP, SERVER_NAME, command_handling};

fn create_response_packet(command: Result<Option<Box<dyn NetworkCommand + Send + Sync>>, Failure>) -> Result<Option<Packet>, Failure> {
    match command? {
        Some(cmd) => {
            Ok(Some(Packet {
                version: NETWORK_VERSION,
                username: SERVER_NAME.to_compact_string(),
                group_id: SERVER_GROUP,
                payload: cmd
            }))
        },
        None => {Ok(None)}
    }
}

pub fn handle_packet(packet: Packet, user_id: &u64) -> Result<Option<Packet>, Failure> {
    match packet.payload.number() {
        DefaultCommand::ID => {Ok(Some(Packet::new(SERVER_NAME, SERVER_GROUP, Box::new(DefaultCommand{}))?))},
        GetOnlineUsers::ID => {
            let outgoing = command_handling::handle_get_online_users(packet);
            create_response_packet(outgoing)
        },
        SetEchoAvailability::ID => {
            let outgoing = command_handling::handle_set_echo_availability(packet, user_id);
            create_response_packet(outgoing)
        },
        SetOnlineStatus::ID => {
            let outgoing = command_handling::handle_set_online_status(packet, user_id);
            create_response_packet(outgoing)
        },
        CreateNewGroup::ID => {
            let outgoing = command_handling::handle_create_new_group(packet, user_id);
            create_response_packet(outgoing)
        },
        JoinGroup::ID => {
            let outgoing = command_handling::handle_join_group(packet, user_id);
            create_response_packet(outgoing)
        },
        GetVisibleGroups::ID => {
            let outgoing = command_handling::handle_get_visible_groups(packet);
            create_response_packet(outgoing)
        }
        _ => {Err(Failure::from((anyhow!("packet handling error: command not found"), FailureType::Warning)))}
    }
}
