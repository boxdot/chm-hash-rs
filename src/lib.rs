use crate::hash::RandomHash;

mod graph;
mod hash;

#[derive(Debug)]
pub struct PerfectHash {
    salt1: Vec<usize>,
    salt2: Vec<usize>,
    graph: Vec<usize>,
}

impl PerfectHash {
    pub fn new(salt1: Vec<usize>, salt2: Vec<usize>, graph: Vec<usize>) -> Self {
        assert_eq!(salt1.len(), salt2.len());
        Self {
            salt1,
            salt2,
            graph,
        }
    }

    pub fn hash<K: AsRef<[u8]>>(&self, key: K) -> usize {
        let hash_f = |salt: &[usize]| -> usize {
            let bytes = key.as_ref().iter().enumerate();
            let hash: usize = bytes.map(|(i, b)| salt[i % salt.len()] * *b as usize).sum();
            hash % self.graph.len()
        };
        (self.graph[hash_f(&self.salt1)] + self.graph[hash_f(&self.salt2)]) % self.graph.len()
    }
}

pub fn generate_hash<Hash, K, I>(mut keys: I) -> (Vec<usize>, Vec<usize>, Vec<usize>)
where
    Hash: RandomHash,
    K: AsRef<[u8]>,
    I: ExactSizeIterator<Item = (K, usize)> + Clone,
{
    const TRIALS: usize = 5;

    let mut n = keys.clone().map(|(_, val)| val + 1).max().unwrap_or(1) as usize;

    let mut trial = 0;
    let (mut f1, mut f2, vertex_values) = loop {
        if trial % TRIALS == 0 {
            if trial > 0 {
                n = (n + 1).max((1.05 * n as f64) as usize);
            }
        }
        trial += 1;

        let mut g = graph::Graph::new(n);
        let mut f1 = Hash::new(n);
        let mut f2 = Hash::new(n);

        for (key, hashval) in keys.clone() {
            g.connect(f1.call(&key) as usize, f2.call(&key) as usize, hashval);
        }

        if let Some(vertex_values) = g.assign_vertex_values() {
            break (f1, f2, vertex_values);
        }
    };

    debug_assert!(
        keys.all(|(key, hashval)| (vertex_values[f1.call(&key) as usize]
            + vertex_values[f2.call(&key) as usize])
            % n
            == hashval)
    );

    (f1.into_salt(), f2.into_salt(), vertex_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hash_for_small_input() {
        let animals = ["Elephant", "Horse", "Camel", "Python", "Dog", "Cat"];

        let (f1, f2, values) = generate_hash::<hash::Hash2, _, _>(
            animals
                .iter()
                .enumerate()
                .map(|(idx, animal)| (animal, idx)),
        );
        let h = PerfectHash::new(f1, f2, values);
        assert!(animals
            .iter()
            .enumerate()
            .all(|(idx, animal)| h.hash(animal) == idx));
    }

    #[test]
    fn test_generate_hash_for_us_states() {
        let us_states = [
            "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN",
            "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV",
            "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN",
            "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY",
        ];

        let (f1, f2, values) = generate_hash::<hash::Hash2, _, _>(
            us_states
                .iter()
                .enumerate()
                .map(|(idx, state)| (state, idx)),
        );
        let h = PerfectHash::new(f1, f2, values);
        assert!(us_states
            .iter()
            .enumerate()
            .all(|(idx, state)| h.hash(state) == idx));
    }
}
