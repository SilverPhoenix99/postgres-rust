pub struct Map<K, V, const SALTS_SIZE: usize>
where
    K: 'static,
    V: 'static,
{
    salts: &'static [i16; SALTS_SIZE],
    entries: &'static [(K, V)],
}

impl<K, V, const SALTS_SIZE: usize> Map<K, V, SALTS_SIZE>
where
    K: MphfHash
{
    // Ensure tables don't exceed 65K entries.
    const _MAX_SALTS_SIZE: () = assert!(SALTS_SIZE <= u16::MAX as usize);

    pub const fn new(salts: &'static [i16; SALTS_SIZE], entries: &'static [(K, V)]) -> Self {
        Self { salts, entries }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: PartialEq<K> + MphfHash
    {
        let index = self.index_of(key)?;
        let (entry_key, value) = &self.entries[index];

        if key == entry_key {
            Some(value)
        }
        else {
            None
        }
    }

    fn index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: PartialEq<K> + MphfHash
    {
        let index = {
            let hasher = self.hasher(0);
            key.mphf_hash(&hasher)
        };
        let salt = self.salts[index];
        if salt == 0 {
            return None
        }

        let index = if salt < 0 {
            // Negative slots are directly indexed.
            // 1 was subtracted to ensure it wasn't confused with slot 0,
            // so now it needs to subtract again.
            (-salt - 1) as usize
        }
        else {
            // 1 was added to ensure it wasn't confused with the empty slot (0),
            // so now it needs to subtract again.
            let salt = (salt - 1) as u32;
            let hasher = self.hasher(salt);
            key.mphf_hash(&hasher)
        };

        Some(index)
    }

    fn hasher(&self, salt: u32) -> Fnv1a<SALTS_SIZE> {
        Fnv1a::new(salt)
    }
}

pub struct Fnv1a<const SALTS_SIZE: usize> {
    salt: Wrapping<u32>
}

impl<const SALTS_SIZE: usize> Fnv1a<SALTS_SIZE> {

    const NUM_BITS: u32 = u32::BITS - (SALTS_SIZE as u32).leading_zeros();

    fn new(salt: u32) -> Self {
        Self {
            salt: Wrapping(salt)
        }
    }

    /// Modified version of FNV-1a, with an extra salt mixed in
    pub fn hash_bytes_iter(&self, bytes: impl Iterator<Item = u8>) -> usize {

        const FNV_PRIME: Wrapping<u32> = Wrapping(0x0100_0193);
        const FNV_OFFSET_BASIS: Wrapping<u32> = Wrapping(0x811c_9dc5);

        let mut hash = bytes.map(|b| Wrapping(b as u32))
            .fold(FNV_OFFSET_BASIS, |acc, b| {
                (b ^ (acc + self.salt)) * FNV_PRIME
            })
            .0;

        // xor-shift excess bits
        let mut n = u32::BITS >> 1;
        let mut mask = (1u32 << n) - 1;
        while n > Self::NUM_BITS {
            hash = (hash >> n) ^ (hash & mask);
            n >>= 1;
            mask >>= n;
        }

        // lazy mod
        (hash % (SALTS_SIZE as u32)) as usize
    }

    pub fn hash_bytes(&self, bytes: &[u8]) -> usize {
        self.hash_bytes_iter(bytes.iter().copied())
    }
}

pub trait MphfHash {
    fn mphf_hash<const SALTS_SIZE: usize>(&self, hasher: &Fnv1a<SALTS_SIZE>) -> usize;
}

impl MphfHash for u64 {
    fn mphf_hash<const SALTS_SIZE: usize>(&self, hasher: &Fnv1a<SALTS_SIZE>) -> usize {
        let bytes = (*self).to_le_bytes();
        hasher.hash_bytes(&bytes)
    }
}

impl MphfHash for u32 {
    fn mphf_hash<const SALTS_SIZE: usize>(&self, hasher: &Fnv1a<SALTS_SIZE>) -> usize {
        let bytes = (*self).to_le_bytes();
        hasher.hash_bytes(&bytes)
    }
}

impl MphfHash for str {
    fn mphf_hash<const SALTS_SIZE: usize>(&self, hasher: &Fnv1a<SALTS_SIZE>) -> usize {
        hasher.hash_bytes((*self).as_bytes())
    }
}

impl MphfHash for &str {
    fn mphf_hash<const SALTS_SIZE: usize>(&self, hasher: &Fnv1a<SALTS_SIZE>) -> usize {
        (*self).mphf_hash(hasher)
    }
}

impl MphfHash for String {
    fn mphf_hash<const SALTS_SIZE: usize>(&self, hasher: &Fnv1a<SALTS_SIZE>) -> usize {
        self.as_str().mphf_hash(hasher)
    }
}

use core::num::Wrapping;
