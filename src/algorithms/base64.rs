use base64::engine::general_purpose as bs64;
use base64::Engine;
use rand::RngCore;

pub fn base64(bytes: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut key = vec![0u8; bytes];
    rng.fill_bytes(&mut key);
    bs64::STANDARD.encode(key)
}
