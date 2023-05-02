use hex as base16;
use rand::RngCore;

pub fn hex(bytes: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut key = vec![0u8; bytes];
    rng.fill_bytes(&mut key);
    base16::encode(key)
}
