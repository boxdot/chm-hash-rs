use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

#[derive(Debug)]
pub struct BytesHash {
    n: usize,
    salt: Vec<u64>,
}

impl BytesHash {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            salt: Vec::new(),
        }
    }

    pub fn call<K: AsRef<[u8]>>(&mut self, key: K) -> u64 {
        if self.salt.len() < key.as_ref().len() {
            let mut rng = thread_rng();
            self.salt.extend(
                Uniform::from(0..self.n as u64)
                    .sample_iter(&mut rng)
                    .take(key.as_ref().len() - self.salt.len()),
            );
        }

        let salt: u64 = self
            .salt
            .iter()
            .zip(key.as_ref())
            .map(|(&s, &c)| s * u64::from(c))
            .sum();
        salt % self.n as u64
    }

    pub fn into_salt(self) -> Vec<usize> {
        self.salt.into_iter().map(|b| b as usize).collect()
    }
}
