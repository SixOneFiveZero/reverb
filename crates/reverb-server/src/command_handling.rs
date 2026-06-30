// cast boxed NetworkCommand into a specific network command, then handle that command

use std::{collections::{HashMap, HashSet}, sync::atomic::Ordering};

use crate::{GROUPS, NEXT_GROUP_ID, ONLINE_USERS, OPEN_USERS, network::group::{Group, add_group}};

use anyhow::anyhow;

use compact_str::ToCompactString;
use reverb_core::{failure::failure::{Failure, FailureType}, network::*, network_command::{create_new_group::CreateNewGroup, failure::NetworkFailure, group_info::GroupInfo, helpers::NetworkCommand, join_group::JoinGroup, online_users::OnlineUsers, set_echo_availability::SetEchoAvailability, set_online_status::SetOnlineStatus}};
use crate::USERS;

// helpers

fn try_get_set_echo_availability(item: &Box<dyn NetworkCommand + Send + Sync>) -> Result<SetEchoAvailability, Failure> {
    if let Some(command) = item.as_any().downcast_ref::<SetEchoAvailability>() {
        Ok(command.clone())
    } else {
        Err(Failure::from((anyhow!("failed to read SetEchoAvailability from Box"), FailureType::Warning)))
    }
}
fn try_get_set_online_status(item: &Box<dyn NetworkCommand + Send + Sync>) -> Result<SetOnlineStatus, Failure> {
    if let Some(command) = item.as_any().downcast_ref::<SetOnlineStatus>() {
        Ok(command.clone())
    } else {
        Err(Failure::from((anyhow!("failed to read SetOnlineStatus from Box"), FailureType::Warning)))
    }
}
fn try_get_create_new_group(item: &Box<dyn NetworkCommand + Send + Sync>) -> Result<CreateNewGroup, Failure> {
    if let Some(command) = item.as_any().downcast_ref::<CreateNewGroup>() {
        Ok(command.clone())
    } else {
        Err(Failure::from((anyhow!("failed to read CreateNewGroup from Box"), FailureType::Warning)))
    }
}
fn try_get_join_group(item: &Box<dyn NetworkCommand + Send + Sync>) -> Result<JoinGroup, Failure> {
    if let Some(command) = item.as_any().downcast_ref::<JoinGroup>() {
        Ok(command.clone())
    } else {
        Err(Failure::from((anyhow!("failed to read JoinGroup from Box"), FailureType::Warning)))
    }
}

// handlers

pub fn handle_get_online_users(_packet: Packet) -> Result<Option<Box<dyn NetworkCommand + Send + Sync>>, Failure> {
    let online_users = ONLINE_USERS.iter()
        .filter_map(|id_ref| {
            let id = *id_ref.key();
            USERS.get(&id).map(|user_ref| {
                (user_ref.value().username().to_compact_string(), user_ref.value().user_info())
            })
        })
        .collect();
    
    Ok(Some(Box::new(OnlineUsers { users: online_users })))
}

pub fn handle_set_echo_availability(packet: Packet, user_id: &u64) -> Result<Option<Box<dyn NetworkCommand + Send + Sync>>, Failure> {
    let command = try_get_set_echo_availability(packet.payload())?;
    let new_status = command.0;
    if let Some(mut user) = USERS.get_mut(user_id) {
        user.value_mut().set_echo_status(new_status);
        match new_status {
            true => {OPEN_USERS.insert(*user_id);},
            false => {OPEN_USERS.remove(user_id);},
        }
        return Ok(None);
    }

    Err(Failure::from((anyhow!("failed to set echo availability for user_id: {user_id}"), FailureType::Warning)))
}

pub fn handle_set_online_status(packet: Packet, user_id: &u64) -> Result<Option<Box<dyn NetworkCommand + Send + Sync>>, Failure> {
    let command = try_get_set_online_status(packet.payload())?;
    let new_status = command.0;
    if let Some(mut user) = USERS.get_mut(user_id) {
        user.value_mut().set_online_status(new_status);
        match new_status {
            true => {ONLINE_USERS.insert(*user_id);},
            false => {ONLINE_USERS.remove(user_id);},
        }
        return Ok(None);
    }

    Err(Failure::from((anyhow!("failed to set online status for user_id: {user_id}"), FailureType::Warning)))
}

pub fn handle_create_new_group(packet: Packet, user_id: &u64) -> Result<Option<Box<dyn NetworkCommand + Send + Sync>>, Failure> {
    let command = try_get_create_new_group(packet.payload())?;
    let host_name = USERS.get(user_id).unwrap().value().username().clone();

    let group_id = NEXT_GROUP_ID.fetch_add(1, Ordering::Relaxed); // wraps around when full overwriting existing groups
    let mut users = HashMap::new();
    users.insert(*user_id, host_name);
    let group = Group {
        id: group_id,
        group_name: command.group_name.clone(),
        users,
        host: *user_id,
        open: command.open,
        visible: command.visible,
    };
    add_group(group_id, group.clone());
    
    Ok(Some(Box::new(
        group.get_info()
    )))
}

pub fn handle_join_group(packet: Packet, user_id: &u64) -> Result<Option<Box<dyn NetworkCommand + Send + Sync>>, Failure> {
    let command = try_get_join_group(packet.payload())?;

    let mut target_group = match GROUPS.get_mut(&command.group_id) {
        Some(group) => group,
        None => {
            let response = NetworkFailure::JoinGroup(format!("no groups with ID {}", command.group_id));
            return Ok(Some(Box::new(response)));
        },
    };

    if !target_group.open {
        let response = NetworkFailure::JoinGroup("group not open to join".to_string());
        return Ok(Some(Box::new(response)));
    }
    
    if let Some(mut user_ref) = USERS.get_mut(user_id) {
        target_group.users.insert(*user_id, user_ref.value().username().clone());
        user_ref.value_mut().update_group_info(target_group.id, target_group.group_name.clone());
    }

    Ok(Some(Box::new(
        target_group.get_info()
    )))
}
