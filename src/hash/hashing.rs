pub const SINGLE_SIZE: usize = 32;
pub const DOUBLE_SIZE: usize = 64;

pub type DSha256 = sha2::Sha256;
pub type DSha512 = sha2::Sha512;
pub type DBlake2s256 = blake2::Blake2s256;
pub type DBlake2b512 = blake2::Blake2b512;

pub trait Hashing: digest::Digest {
    fn hash(data: &[u8]) -> Vec<u8> {
        Self::digest(data).to_vec()
    }
}

impl<T: digest::Digest> Hashing for T {}

#[cfg(test)]
mod test {
    use digest::OutputSizeUser;

    use crate::DBlake3;

    use super::*;

    #[test]
    fn test_size_method() {
        assert_eq!(DSha256::output_size(), SINGLE_SIZE);
        assert_eq!(DSha512::output_size(), DOUBLE_SIZE);
        assert_eq!(DBlake2b512::output_size(), DOUBLE_SIZE);
        assert_eq!(DBlake2s256::output_size(), SINGLE_SIZE);
        assert_eq!(DBlake3::output_size(), SINGLE_SIZE);
    }
}
