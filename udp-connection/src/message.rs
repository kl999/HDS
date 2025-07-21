use std::fmt;
use sha2::{Sha256, Digest};

pub struct Message {
    pub id: u64,
    pub hash: Box<[u8]>,
    pub data: Box<[u8]>,
}

impl Message {
    pub fn new(id: u64, data: Box<[u8]>) -> Message {
        let mut hasher = Sha256::new();
        hasher.update(&id.to_be_bytes());
        hasher.update(&data);
        let hash: Box<[u8]> = hasher.finalize().to_vec().into_boxed_slice();
        
        Message {
            id,
            hash: hash,
            data,
        }
    }

    pub fn deserialize(ser: &[u8]) -> Message {
        todo!()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "#{} ({:x?}): {}", self.id, self.hash, String::from_utf8_lossy(&self.data))
    }
}
