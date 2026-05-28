use crate::{ONLINE_USERS, OPEN_USERS};

use anyhow::anyhow;

use compact_str::ToCompactString;
use reverb_core::{failure::failure::{Failure, FailureType}, network::*, network_command::{helpers::NetworkCommand, online_users::OnlineUsers, set_echo_availability::SetEchoAvailability}};
use crate::USERS;

pub fn handle_get_online_users(_packet: Packet) -> Box<dyn NetworkCommand + Send + Sync> {
    let online_users = ONLINE_USERS.iter()
        .filter_map(|id_ref| {
            let id = *id_ref.key();
            USERS.get(&id).map(|user_ref| {
                (user_ref.value().username().to_compact_string(), user_ref.value().user_info(id))
            })
        })
        .collect();
    
    Box::new(OnlineUsers { users: online_users }) 
}

fn try_get_set_echo_availability(item: &Box<dyn NetworkCommand + Send + Sync>) -> Result<SetEchoAvailability, Failure> {
    if let Some(command) = item.as_any().downcast_ref::<SetEchoAvailability>() {
        Ok(command.clone())
    } else {
        Err(Failure::from((anyhow!("failed to read SetEchoAvailability from Box"), FailureType::Warning)))
    }
}

pub fn handle_set_echo_availability(packet: Packet, user_id: &u64) -> Result<(), Failure> {
    let command = try_get_set_echo_availability(packet.payload())?;
    let new_status = command.0;
    if let Some(mut user) = USERS.get_mut(user_id) {
        user.value_mut().set_echo_status(new_status);
        match new_status {
            true => {OPEN_USERS.insert(*user_id);},
            false => {OPEN_USERS.remove(user_id);},
        }
        return Ok(());
    }

    Err(Failure::from((anyhow!("failed to set echo availability for user_id: {user_id}"), FailureType::Warning)))
}
