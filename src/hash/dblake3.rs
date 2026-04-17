use digest::{
    Digest, FixedOutput, FixedOutputReset, Output, OutputSizeUser, Reset,
    Update,
};
use typenum::U32;

pub struct DBlake3(blake3::Hasher);

impl OutputSizeUser for DBlake3 {
    type OutputSize = U32;
}

impl Reset for DBlake3 {
    fn reset(&mut self) {
        self.0 = blake3::Hasher::new();
    }
}

impl Update for DBlake3 {
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

impl FixedOutput for DBlake3 {
    fn finalize_into(self, out: &mut Output<Self>) {
        let hash = self.0.finalize();
        out.copy_from_slice(hash.as_bytes());
    }
}

impl FixedOutputReset for DBlake3 {
    fn finalize_into_reset(&mut self, out: &mut Output<Self>) {
        let hash = self.0.finalize();
        out.copy_from_slice(hash.as_bytes());
        self.0 = blake3::Hasher::new(); // Reset after finalize
    }
}

impl Digest for DBlake3 {
    fn new() -> Self {
        Self(blake3::Hasher::new())
    }

    fn new_with_prefix(data: impl AsRef<[u8]>) -> Self {
        let mut hasher = Self::new();
        Digest::update(&mut hasher, data);
        hasher
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        Update::update(self, data.as_ref());
    }

    fn chain_update(self, data: impl AsRef<[u8]>) -> Self {
        let mut hasher = self;
        Digest::update(&mut hasher, data);
        hasher
    }

    fn finalize(self) -> Output<Self> {
        let mut out = Output::<Self>::default();
        FixedOutput::finalize_into(self, &mut out);
        out
    }

    fn finalize_into(self, out: &mut Output<Self>) {
        FixedOutput::finalize_into(self, out);
    }

    fn finalize_reset(&mut self) -> Output<Self>
    where
        Self: FixedOutputReset,
    {
        let mut out = Output::<Self>::default();
        FixedOutputReset::finalize_into_reset(self, &mut out);
        out
    }

    fn finalize_into_reset(&mut self, out: &mut Output<Self>)
    where
        Self: FixedOutputReset,
    {
        FixedOutputReset::finalize_into_reset(self, out);
    }

    fn reset(&mut self)
    where
        Self: Reset,
    {
        Reset::reset(self);
    }

    fn output_size() -> usize {
        32
    }

    fn digest(data: impl AsRef<[u8]>) -> Output<Self> {
        let hash = blake3::hash(data.as_ref());
        let mut out = Output::<Self>::default();
        out.copy_from_slice(hash.as_bytes());
        out
    }
}
