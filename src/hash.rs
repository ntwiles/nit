use sha2::{Digest, Sha256};
use std::fmt::Write;

pub fn sha_256_hex(data: &str) -> String {
    let hash = Sha256::new().chain_update(data).finalize();

    let mut hex = String::with_capacity(64);
    for num in hash {
        write!(&mut hex, "{:02x}", num).unwrap();
    }

    hex
}
