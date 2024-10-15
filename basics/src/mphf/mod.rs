
pub struct Map<K, V>
where
    K: 'static,
    V: 'static,
{
    salts: &'static [i16],
    entries: &'static [(K, V)],
}

impl<K, V> Map<K, V>
where
    K: MphfHash
{
    pub const fn new(salts: &'static [i16], entries: &'static [(K, V)]) -> Self {
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
            let salt = (salt - 1) as u64;
            let hasher = self.hasher(salt);
            key.mphf_hash(&hasher)
        };

        Some(index)
    }

    fn hasher(&self, salt: u64) -> Fnv1a {
        Fnv1a::new(salt, self.salts.len() as u64)
    }
}

const FNV_PRIME: Wrapping<u64> = Wrapping(0x0100_0000_01b3);
const FNV_OFFSET_BASIS: Wrapping<u64> = Wrapping(0xcbf2_9ce4_8422_2325);

pub trait MphfHasher {
    fn hash_bytes_iter(&self, bytes: impl Iterator<Item = u8>) -> usize;

    fn hash_bytes(&self, bytes: &[u8]) -> usize {
        self.hash_bytes_iter(bytes.iter().copied())
    }
}

pub struct Fnv1a {
    salt: Wrapping<u64>,
    table_size: u64,
}

impl Fnv1a {
    fn new(salt: u64, table_size: u64) -> Self {
        Self {
            salt: Wrapping(salt),
            table_size,
        }
    }
}

impl MphfHasher for Fnv1a {
    /// Modified version of FNV-1a, with an extra salt mixed in
    fn hash_bytes_iter(&self, bytes: impl Iterator<Item = u8>) -> usize {

        let hash = bytes.map(|b| Wrapping(b as u64))
            .fold(FNV_OFFSET_BASIS, |acc, b| {
                (b ^ (acc + self.salt)) * FNV_PRIME
            });

        (hash.0 % self.table_size) as usize
    }
}

pub trait MphfHash {
    fn mphf_hash(&self, hasher: &impl MphfHasher) -> usize;
}

impl MphfHash for u64 {
    fn mphf_hash(&self, hasher: &impl MphfHasher) -> usize {
        let bytes = (*self).to_le_bytes();
        hasher.hash_bytes(&bytes)
    }
}

impl MphfHash for u32 {
    fn mphf_hash(&self, hasher: &impl MphfHasher) -> usize {
        let bytes = (*self).to_le_bytes();
        hasher.hash_bytes(&bytes)
    }
}

impl MphfHash for str {
    fn mphf_hash(&self, hasher: &impl MphfHasher) -> usize {
        hasher.hash_bytes((*self).as_bytes())
    }
}

impl MphfHash for &str {
    fn mphf_hash(&self, hasher: &impl MphfHasher) -> usize {
        (*self).mphf_hash(hasher)
    }
}

impl MphfHash for String {
    fn mphf_hash(&self, hasher: &impl MphfHasher) -> usize {
        self.as_str().mphf_hash(hasher)
    }
}

use std::num::Wrapping;
