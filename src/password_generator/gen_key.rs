use rand::{rngs, RngCore};


pub fn generate_ase_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    let mut rng = rngs::OsRng;

    rng.fill_bytes(&mut key);

    key
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_key() {
        let key1 = generate_ase_key();
        let key2 = generate_ase_key();

        assert_ne!(key1, key2);
    }
}