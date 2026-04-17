#[cfg(test)]
mod tests {
    use arx_core::{
        DBlake2b512, DBlake2s256, DBlake3, DOUBLE_SIZE, DSha256, DSha512,
        Hashing, SINGLE_SIZE,
    };
    use rand::Rng;

    const TEST_DATA: &[u8] = b"Hello, World!";

    fn test_empty_data_t<H: Hashing>() {
        let hash = H::hash(b"");
        assert!(!hash.is_empty())
    }

    #[test]
    fn test_empty_data() {
        test_empty_data_t::<DSha256>();
        test_empty_data_t::<DSha512>();
        test_empty_data_t::<DBlake2s256>();
        test_empty_data_t::<DBlake2b512>();
        test_empty_data_t::<DBlake3>();
    }

    fn test_incremental_hashing_t<H: Hashing>(test_data: &[u8]) {
        let length = test_data.len();
        let n = rand::thread_rng().gen_range(0..length);
        let data_parts: &[&[u8]] = &[&test_data[..n], &test_data[n..]];

        let direct_hash = H::hash(test_data);

        let mut incremental_hasher = H::new();
        for part in data_parts {
            incremental_hasher.update(part);
        }
        let incremental_hash = incremental_hasher.finalize().to_vec();

        assert_eq!(direct_hash, incremental_hash);
    }

    #[test]
    fn test_incremental_hashing() {
        test_incremental_hashing_t::<DSha256>(TEST_DATA);
        test_incremental_hashing_t::<DSha512>(TEST_DATA);
        test_incremental_hashing_t::<DBlake2s256>(TEST_DATA);
        test_incremental_hashing_t::<DBlake2b512>(TEST_DATA);
        test_incremental_hashing_t::<DBlake3>(TEST_DATA);
    }

    macro_rules! hash_all {
        ($test_data:expr, $($hash_type:ty),+) => {
            [
                $(<$hash_type as Hashing>::hash($test_data)),+
            ]
        };
    }

    macro_rules! assert_all_different {
        ($($hash:expr),+) => {
            let hashes = [$($hash),+];
            for i in 0..hashes.len() {
                for j in (i+1)..hashes.len() {
                    assert_ne!(hashes[i], hashes[j]);
                }
            }
        };
    }

    fn test_different_hashes_for_different_algorithms_t<H1, H2, H3, H4, H5>(
        test_data: &[u8],
    ) where
        H1: Hashing,
        H2: Hashing,
        H3: Hashing,
        H4: Hashing,
        H5: Hashing,
    {
        let hashes = hash_all!(test_data, H1, H2, H3, H4, H5);
        assert_all_different!(&hashes);
    }

    #[test]
    fn test_different_hashes_for_different_algorithms() {
        test_different_hashes_for_different_algorithms_t::<
            DSha256,
            DSha512,
            DBlake2s256,
            DBlake2b512,
            DBlake3,
        >(TEST_DATA);
    }

    fn test_deterministic_t<H: Hashing>(test_data: &[u8]) {
        let hash1 = H::hash(test_data);
        let hash2 = H::hash(test_data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_deterministic() {
        test_deterministic_t::<DSha256>(TEST_DATA);
        test_deterministic_t::<DSha512>(TEST_DATA);
        test_deterministic_t::<DBlake2s256>(TEST_DATA);
        test_deterministic_t::<DBlake2b512>(TEST_DATA);
        test_deterministic_t::<DBlake3>(TEST_DATA);
    }

    fn test_large_data_t<H: Hashing, const OUT_SIZE: usize>(data: &[u8]) {
        let hash = H::hash(data);
        assert_eq!(hash.len(), OUT_SIZE);
    }

    #[test]
    fn test_large_data() {
        let large_data = vec![0x42; 1024 * 1024];

        test_large_data_t::<DSha256, SINGLE_SIZE>(&large_data);
        test_large_data_t::<DSha512, DOUBLE_SIZE>(&large_data);
        test_large_data_t::<DBlake2s256, SINGLE_SIZE>(&large_data);
        test_large_data_t::<DBlake2b512, DOUBLE_SIZE>(&large_data);
        test_large_data_t::<DBlake3, SINGLE_SIZE>(&large_data);
    }

    fn test_chained_updates_t<H: Hashing>(n: usize) {
        let mut hasher = H::new();

        for i in 0..n {
            hasher.update(&[i as u8]);
        }

        let hash_chained = hasher.finalize().to_vec();

        let all_bytes: Vec<u8> = (0..n).map(|i| i as u8).collect();
        let hash_direct = H::hash(&all_bytes);

        assert_eq!(hash_chained, hash_direct);
    }

    #[test]
    fn test_chained_updates() {
        test_chained_updates_t::<DSha256>(100);
        test_chained_updates_t::<DSha512>(100);
        test_chained_updates_t::<DBlake2s256>(100);
        test_chained_updates_t::<DBlake2b512>(100);
        test_chained_updates_t::<DBlake3>(100);
    }
}
