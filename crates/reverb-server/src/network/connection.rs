// all networking/connection logic

use anyhow::anyhow;
use quinn::{Connection, Incoming, RecvStream};

use reverb_core::{failure::failure::{Failure, FailureType}, network::*};
use crate::network::{packet_handling::handle_packet};
use crate::user::{register_new_user, remove_user};

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


