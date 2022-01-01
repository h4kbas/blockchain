pub type BlockHash = Vec<u8>;

use std::convert::TryInto;
use std::time::SystemTime;

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

pub fn u32_bytes(u: &u32) -> [u8; 4] {
    u.to_le_bytes()
}
pub fn u64_bytes(u: &u64) -> [u8; 8] {
    u.to_le_bytes()
}
pub fn u128_bytes(u: &u128) -> [u8; 16] {
    u.to_le_bytes()
}

pub fn difficulty_as_bytes_u128(v: &Vec<u8>) -> u128 {
    u128::from_le_bytes(v.as_slice()[0..16].try_into().unwrap())
}

mod block;
pub use crate::block::Block;
mod hashable;
pub use crate::hashable::Hashable;
