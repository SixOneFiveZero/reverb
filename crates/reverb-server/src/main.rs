use std::sync::{LazyLock, atomic::{AtomicU32, AtomicU64}};
use dashmap::{DashMap, DashSet};
use quinn::Endpoint;

use reverb_core::failure::failure::Failure;
use crate::{group::Group, network::connection, user::User};

mod network;
mod server_startup;
mod command_handling;
mod user;
mod group;


// The address and port the server will listen on
const LISTEN_ADDR: &str = "127.0.0.1:4433";
// The server version, included in responses for client verification
const SERVER_NAME: &str = "server";
const SERVER_GROUP: u32 = 0;

static USERS: LazyLock<DashMap<u64, User>> = LazyLock::new(DashMap::new);
static ONLINE_USERS: LazyLock<DashSet<u64>> = LazyLock::new(DashSet::new);
static OPEN_USERS: LazyLock<DashSet<u64>> = LazyLock::new(DashSet::new);
static NEXT_USER_ID: AtomicU64 = AtomicU64::new(1);

static GROUPS: LazyLock<DashMap<u32, Group>> = LazyLock::new(DashMap::new);
static VISIBLE_GROUPS: LazyLock<DashSet<u32>> = LazyLock::new(DashSet::new);
static OPEN_GROUPS: LazyLock<DashSet<u32>> = LazyLock::new(DashSet::new);
static NEXT_GROUP_ID: AtomicU32 = AtomicU32::new(1);

/// Entry point for the server. Installs the default crypto provider, starts the async runtime,
/// and runs the main server logic. Exits with error code 1 if the server fails at startup or error
/// code 2 if fails at runtime.

#[tokio::main]
async fn main() {
    let _ = rustls::crypto::ring::default_provider().install_default();
    println!("Server starting on {}", LISTEN_ADDR);

    // run server startup
    let endpoint = match server_startup::startup() {
        Ok(endpoint) => endpoint,
        Err(failure) => {
            eprintln!("Server startup error: {failure}");
            std::process::exit(1);
        }
    };

    loop {
        if let Err(e) = run(&endpoint).await {
            eprintln!("Server runtime error: {e}");
            if let Failure::Fatal(_, _) = e {
                std::process::exit(2)
            }
        }
    }

}

// accepts incoming connections and hands them off to new tokio async task
async fn run(endpoint: &Endpoint) -> Result<(), Failure> {
    if let Some(conn) = endpoint.accept().await {
        tokio::spawn(async move {
            if let Err(e) = connection::handle_connection(conn).await {
                eprintln!("Server runtime error: error handling connection: {e}")
            };
        });
    }

    Ok(())
}

