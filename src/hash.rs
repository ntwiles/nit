use sha2::{Digest, Sha256};

pub fn sha_256_hex(data: &str) -> String {
    let hash = Sha256::new().chain_update(data).finalize();

    let mut hex = String::new();
    for num in hash {
        hex += &format!("{:x}", num);
    }

    return hex;
}
