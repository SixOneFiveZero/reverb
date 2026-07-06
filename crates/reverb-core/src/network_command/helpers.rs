use std::any::Any;

use crate::network_command::ID::NetworkCommandID;
use crate::failure::failure::{Failure, FailureType};
use crate::network_command::create_new_group::CreateNewGroup;
use crate::network_command::failure::NetworkFailure;
use crate::network_command::fetch_groups::FetchGroups;
use crate::network_command::fetched_groups::FetchedGroups;
use crate::network_command::group_info::GroupInfo;
use crate::network_command::join_group::JoinGroup;
use crate::network_command::set_online_status::SetOnlineStatus;
use crate::network_command::{default_command::DefaultCommand, echo::Echo, fetch_users::FetchUsers, fetched_users::FetchedUsers, set_echo_availability::SetEchoAvailability, skip::Skip, user_data::UserData};
use anyhow::anyhow;

pub enum QueryOrNotify {
    Query,
    Notify
}

// parse data to the apporpriate command from the netowrk
// TODO: find a better variable name than full_data
pub fn parse_command(full_data: Vec<u8>) -> Result<Box<dyn NetworkCommand + Send + Sync>, Failure> {
    println!("command size: {} bytes", full_data.len()); // Debug line
    let cmd_number = full_data[0];

    let data: Vec<u8> = full_data.into_iter().skip(1).collect();

    match cmd_number {
        DefaultCommand::ID => Ok(Box::new(DefaultCommand::parse(data)?)),
        Skip::ID => Ok(Box::new(Skip::parse(data)?)),
        Echo::ID => Ok(Box::new(Echo::parse(data)?)),
        FetchedUsers::ID => Ok(Box::new(FetchedUsers::parse(data)?)),
        FetchUsers::ID => Ok(Box::new(FetchUsers::parse(data)?)),
        UserData::ID => Ok(Box::new(UserData::parse(data)?)),
        SetEchoAvailability::ID => Ok(Box::new(SetEchoAvailability::parse(data)?)),
        SetOnlineStatus::ID => Ok(Box::new(SetOnlineStatus::parse(data)?)),
        CreateNewGroup::ID => Ok(Box::new(CreateNewGroup::parse(data)?)),
        NetworkFailure::ID => Ok(Box::new(NetworkFailure::parse(data)?)),
        FetchGroups::ID => Ok(Box::new(FetchGroups::parse(data)?)),
        FetchedGroups::ID => Ok(Box::new(FetchedGroups::parse(data)?)),
        GroupInfo::ID => Ok(Box::new(GroupInfo::parse(data)?)),
        JoinGroup::ID => Ok(Box::new(JoinGroup::parse(data)?)),
        _ => Err(Failure::from((anyhow!("invalid command"), FailureType::Warning)))
    }
}

// serialize a command to be sent over the network
pub fn serialize(boxed_cmd: &Box<dyn NetworkCommand + Send + Sync>) -> Result<Vec<u8>, Failure> {
    let mut data = vec![boxed_cmd.number()];
    data.append(&mut boxed_cmd.serialize()?);
    Ok(data)
}

pub trait NetworkCommand: Any {
    fn number(&self) -> u8; // numbers should be changed when any functionality changes as we are NOT maintaining backwards compatability
    fn serialize(&self) -> Result<Vec<u8>, Failure>;
    fn parse(data: Vec<u8>) -> Result<Self, Failure> where Self: Sized;
    fn query_or_notify(&self) -> QueryOrNotify;
    fn as_any(&self) -> &dyn Any;
}

