use cosmwasm_std::Addr;
use sha2::{Digest, Sha256};

pub fn flip_coin(height: u64, sender: Addr, timestamp_nanos: u64) -> bool {
    let mut hasher = Sha256::new();

    let block_height_bytes = height.to_be_bytes();
    let sender_bytes = sender.as_bytes();
    let time_nanos_bytes = timestamp_nanos.to_be_bytes();

    hasher.update(block_height_bytes);
    hasher.update(sender_bytes);
    hasher.update(time_nanos_bytes);

    let hash = hasher.finalize();
    let is_head = hash[0] < 128;

    is_head
}
