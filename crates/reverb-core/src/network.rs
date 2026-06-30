use std::fmt;

use crate::{network_command::{default_command::DefaultCommand, helpers::{NetworkCommand, parse_command, serialize}}, failure::failure::{Failure, FailureType}}; 
use anyhow::anyhow;
use compact_str::{CompactString, ToCompactString};


// Major release when there is a breaking change to the packet structure or protocol.
//  e.g. changing header fields, removing possible functions from payload ect.
// Minor release when there is a non-breaking change to the packet structure or protocol that is backwards compatible.
//  e.g. adding new possible functions to payload ect.
// Patch release when there is a change to the packet structure or protocol which is backwards compatible and does not add any new features.
//  e.g. fixing a bug, changing error messages, changing a functions internals, ect.
pub static NETWORK_VERSION: [u8; 3] = [0, 1, 0];


pub struct Packet {
    pub version: [u8; 3],
    pub username: CompactString,
    pub group_id: u32,
    pub payload: Box<dyn NetworkCommand + Send + Sync>,
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Packet")
            .field("version", &self.version)
            .field("username", &self.username)
            .field("group_id", &self.group_id)
            .field("payload_number", &self.payload.number())
            .finish()
    }
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        let payload = parse_command(serialize(&self.payload).unwrap_or_default())
            .unwrap_or_else(|_| Box::new(DefaultCommand {}));

        Packet {
            version: self.version,
            username: self.username.clone(),
            group_id: self.group_id,
            payload,
        }
    }
}

impl Packet {
    pub fn new(
        username: &str,
        group_id: u32,
        payload: Box<dyn NetworkCommand + Send + Sync>,
    ) -> Result<Self, Failure> {
        check_parameters(username)?;

        Ok(Packet {
            version: NETWORK_VERSION,
            username: username.to_compact_string(),
            group_id,
            payload,
        })
    }

    pub fn parse(_data: &[u8]) -> Result<Packet, Failure> {
        println!("data length to parse: {} bytes", _data.len()); // Debug line
        if _data.len() < 41 {
            return Err(Failure::from((anyhow!("Data too short to be a valid packet"), FailureType::Warning)));
        }
        let version = [_data[0], _data[1], _data[2]];
        let username = CompactString::from_utf8_lossy(&_data[3..35]).trim_matches(char::from(0)).into();
        println!("{username}");
        let group_id_bytes: [u8; 4] = _data[35..39].try_into().map_err(|e| Failure::from((anyhow!("{e}"), FailureType::Warning)))?;
        let group_id = u32::from_le_bytes(group_id_bytes);
        println!("{group_id}");
        let payload = parse_command(_data[40..].to_vec())?;

        Ok(Packet {
            version,
            username,
            group_id,
            payload,
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Failure> {
        check_parameters(&self.username)?;
        let mut data = NETWORK_VERSION.to_vec();
        for i in 0..32 {
            if i < self.username.len() {
                data.push(self.username.as_bytes()[i]);
            } else {
                data.push(0);
            }
        }
        data.extend_from_slice(&self.group_id.to_le_bytes());
        data.append(&mut vec![self.payload.number()]);
        data.append(&mut serialize(&self.payload)?);
        Ok(data)
    }

    pub fn version(&self) -> &[u8; 3] {
        &self.version
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn group_id(&self) -> &u32 {
        &self.group_id
    }
    pub fn payload(&self) -> &Box<dyn NetworkCommand + Send + Sync> {
        &self.payload
    }
}

fn check_parameters(username: &str) -> Result<(), Failure> {
    if username.len() > 32 {
        return Err(Failure::from((anyhow!("username too long"), FailureType::Warning)));
    }
    Ok(())
}
