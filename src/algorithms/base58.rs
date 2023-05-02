use rand::RngCore;

pub fn base58(bytes: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut key = vec![0u8; bytes];
    rng.fill_bytes(&mut key);
    bs58::encode(key).into_string()
}
