use std::{collections::HashSet};
use anyhow::anyhow;
use compact_str::CompactString;
use quinn::{Connection, Incoming, RecvStream};

use reverb_core::{failure::failure::{Failure, FailureType}, network::*, network_command::online_users::UserInfo};
use crate::{ONLINE_USERS, OPEN_USERS, USERS, network::packet_handling::{handle_packet, register_new_user}};

pub async fn handle_connection(conn: Incoming) -> Result<(), Failure> {
    let conn_bi = conn.await
        .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;
    println!("Client connected");
    let conn_uni = conn_bi.clone();

    let user_id = receive_user_info(&conn_uni).await?;

    // separate stream handlers to avoid bidirectional handler stalling unidiractional handler
    tokio::spawn(async move {
        loop {
            if let Err(e) = handle_bi(&conn_bi, &user_id).await {
                eprintln!("Server bi_connection error: {e}");
                remove_user(&user_id);
                return;
            }
        }
    });
    tokio::spawn(async move {
        loop {
            if let Err(e) = handle_uni(&conn_uni, &user_id).await {
                eprintln!("Server uni_connection error: {e}");
                remove_user(&user_id);
                return;
            }
        }
    });

    Ok(())
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
    USERS.remove(user_id);
    ONLINE_USERS.remove(user_id);
    OPEN_USERS.remove(user_id);
}

async fn handle_bi(conn: &Connection, user_id: &u64) -> Result<(), Failure> {
    let (mut send, recv) = conn.accept_bi().await
        .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;

    let data = read_incoming(recv).await?;
    let packet = Packet::parse(&data)?;

    // Prepare and send a response back to the client
    let response = handle_packet(packet, user_id)?
        .ok_or(Failure::from((anyhow!("error creating response packet"), FailureType::Warning)))?;

    send.write_all(&response.serialize()?).await
        .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;
    send.finish()
        .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;
    
    Ok(())
}
async fn handle_uni(conn: &Connection, user_id: &u64) -> Result<(), Failure> {
    let recv = conn.accept_uni().await
        .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;

    let data = read_incoming(recv).await?;
    let packet = Packet::parse(&data)?;
    handle_packet(packet, user_id)?;

    Ok(())
} 

async fn receive_user_info(conn: &Connection) -> Result<u64, Failure> {
    let recv = conn.accept_uni().await
        .map_err(|e| Failure::from((e.into(), FailureType::Warning)))?;
    println!("received user info"); // DEBUG
    let data = read_incoming(recv).await?;
    let packet = Packet::parse(&data)?;
    Ok(register_new_user(packet))
}

async fn read_incoming(mut recv: RecvStream) -> Result<Vec<u8>, Failure> {
    recv.read_to_end(1024).await
        .map_err(|e| Failure::from((e.into(), FailureType::Warning)))
}

#[derive(Debug, Clone)]
pub struct User {
    username: CompactString,
    user_info: UserInfo,
    show_online: bool,
    is_group_visible: bool,
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
}

#[derive(Debug, Clone)]
pub struct Group {
    pub group_name: String,
    pub users: HashSet<u8>,
    pub host: u16,
    pub is_group_open: bool,
    pub is_group_visible: bool,
}
