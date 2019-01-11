use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::thread_rng;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait RandomHash {
    fn new(n: usize) -> Self;
    fn call<K: AsRef<[u8]>>(&mut self, key: K) -> u64;
    fn into_salt(self) -> Vec<usize>;
}

#[derive(Debug)]
pub struct Hash1 {
    n: usize,
    salt: String,
}

impl RandomHash for Hash1 {
    fn new(n: usize) -> Self {
        let mut rng = thread_rng();
        let salt: String = Alphanumeric.sample_iter(&mut rng).take(8).collect();
        Self { n, salt }
    }

    fn call<K: AsRef<[u8]>>(&mut self, key: K) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.salt.hash(&mut hasher);
        key.as_ref().hash(&mut hasher);
        hasher.finish() % (self.n as u64)
    }

    fn into_salt(self) -> Vec<usize> {
        self.salt.as_bytes().iter().map(|&b| b as usize).collect()
    }
}

#[derive(Debug)]
pub struct Hash2 {
    n: usize,
    salt: Vec<u64>,
}

impl RandomHash for Hash2 {
    fn new(n: usize) -> Self {
        Self {
            n,
            salt: Vec::new(),
        }
    }

    fn call<K: AsRef<[u8]>>(&mut self, key: K) -> u64 {
        if self.salt.len() < key.as_ref().len() {
            let mut rng = thread_rng();
            self.salt.extend(
                Uniform::from(0..self.n as u64)
                    .sample_iter(&mut rng)
                    .take(key.as_ref().len() - self.salt.len()),
            );
        }

        self.salt
            .iter()
            .zip(key.as_ref())
            .map(|(&s, &c)| s * c as u64)
            .sum::<u64>()
            % self.n as u64
    }

    fn into_salt(self) -> Vec<usize> {
        self.salt.into_iter().map(|b| b as usize).collect()
    }
}
