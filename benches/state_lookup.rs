use criterion::{criterion_group, criterion_main, Criterion};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use std::collections::HashMap;

const TABLE: [(&str, &str, u32); 50] = [
    ("AL", "Alabama", 4335400),
    ("AK", "Alaska", 611500),
    ("AZ", "Arizona", 4664600),
    ("AR", "Arkansas", 2531000),
    ("CA", "California", 33198100),
    ("CO", "Colorado", 3930700),
    ("CT", "Connecticut", 3271100),
    ("DE", "Delaware", 736900),
    ("FL", "Florida", 15012200),
    ("GA", "Georgia", 7562200),
    ("HI", "Hawaii", 1188400),
    ("ID", "Idaho", 1221500),
    ("IL", "Illinois", 11981700),
    ("IN", "Indiana", 5882500),
    ("IA", "Iowa", 2854700),
    ("KS", "Kansas", 2603200),
    ("KY", "Kentucky", 3921000),
    ("LA", "Louisiana", 4361200),
    ("ME", "Maine", 1243700),
    ("MD", "Maryland", 5122400),
    ("MA", "Massachusetts", 6133500),
    ("MI", "Michigan", 9825100),
    ("MN", "Minnesota", 4704200),
    ("MS", "Mississippi", 2739700),
    ("MO", "Missouri", 5421400),
    ("MT", "Montana", 886400),
    ("NE", "Nebraska", 1661400),
    ("NV", "Nevada", 1828700),
    ("NH", "New Hampshire", 1179100),
    ("NJ", "New Jersey", 8078300),
    ("NM", "New Mexico", 1738700),
    ("NY", "New York", 18197800),
    ("NC", "North Carolina", 7483100),
    ("ND", "North Dakota", 640000),
    ("OH", "Ohio", 11197900),
    ("OK", "Oklahoma", 3328100),
    ("OR", "Oregon", 3266800),
    ("PA", "Pennsylvania", 12044200),
    ("RI", "Rhode Island", 987000),
    ("SC", "South Carolina", 3781800),
    ("SD", "South Dakota", 738500),
    ("TN", "Tennessee", 5398200),
    ("TX", "Texas", 19274300),
    ("UT", "Utah", 2071500),
    ("VT", "Vermont", 590400),
    ("VA", "Virginia", 6768400),
    ("WA", "Washington", 5674900),
    ("WV", "West Virginia", 1813200),
    ("WI", "Wisconsin", 5224500),
    ("WY", "Wyoming", 479500),
];

const SALT1: [u8; 2] = [43, 58];
const SALT2: [u8; 2] = [51, 46];
const GRAPH: [u8; 81] = [
    0, 0, 0, 12, 64, 0, 0, 0, 0, 4, 0, 22, 0, 0, 12, 25, 45, 0, 0, 13, 0, 0, 43, 37, 0, 27, 0, 0,
    8, 15, 0, 32, 7, 0, 13, 0, 16, 35, 0, 0, 3, 7, 1, 0, 46, 0, 19, 76, 0, 13, 77, 31, 35, 9, 62,
    0, 13, 0, 40, 33, 0, 26, 51, 70, 7, 38, 26, 0, 65, 6, 29, 14, 47, 0, 49, 0, 21, 71, 28, 0, 37,
];

fn hash_f1<K: AsRef<[u8]>>(key: K) -> usize {
    let bytes = key.as_ref().iter().enumerate();
    let hash: usize = bytes
        .map(|(i, b)| SALT1[i % SALT1.len()] as usize * *b as usize)
        .sum();
    hash % GRAPH.len()
}

fn hash_f2<K: AsRef<[u8]>>(key: K) -> usize {
    let bytes = key.as_ref().iter().enumerate();
    let hash: usize = bytes
        .map(|(i, b)| SALT2[i % SALT2.len()] as usize * *b as usize)
        .sum();
    hash % GRAPH.len()
}

pub fn hash<K: AsRef<[u8]>>(key: K) -> usize {
    (GRAPH[hash_f1(&key)] as usize + GRAPH[hash_f2(&key)] as usize) % GRAPH.len()
}

fn static_perfect_hash_lookup(c: &mut Criterion) {
    let mut rng: StdRng = SeedableRng::seed_from_u64(42);
    let mut keys: Vec<_> = TABLE
        .iter()
        .enumerate()
        .map(|(idx, (abbr, _, _))| (abbr, idx))
        .collect();
    keys.shuffle(&mut rng);

    c.bench_function("static perfect hash lookup", move |b| {
        b.iter(move || {
            assert_eq!(TABLE[hash("NV")].1, TABLE[27].1);
            assert_eq!(TABLE[hash("KS")].1, TABLE[15].1);
            assert_eq!(TABLE[hash("TN")].1, TABLE[41].1);
            assert_eq!(TABLE[hash("GA")].1, TABLE[9].1);
            assert_eq!(TABLE[hash("MA")].1, TABLE[20].1);
            assert_eq!(TABLE[hash("VA")].1, TABLE[45].1);
            assert_eq!(TABLE[hash("KY")].1, TABLE[16].1);
            assert_eq!(TABLE[hash("NM")].1, TABLE[30].1);
            assert_eq!(TABLE[hash("WA")].1, TABLE[46].1);
            assert_eq!(TABLE[hash("UT")].1, TABLE[43].1);
        })
    });
}

fn perfect_hash_lookup(c: &mut Criterion) {
    let mut keys: Vec<_> = TABLE
        .iter()
        .enumerate()
        .map(|(idx, (abbr, _, _))| (abbr, idx))
        .collect();

    let h = chm92::generate_hash(keys.iter().cloned());

    let mut rng: StdRng = SeedableRng::seed_from_u64(42);
    keys.shuffle(&mut rng);

    c.bench_function("perfect hash lookup", move |b| {
        let h = h.clone();
        b.iter(move || {
            assert_eq!(TABLE[h.hash("NV")].1, TABLE[27].1);
            assert_eq!(TABLE[h.hash("KS")].1, TABLE[15].1);
            assert_eq!(TABLE[h.hash("TN")].1, TABLE[41].1);
            assert_eq!(TABLE[h.hash("GA")].1, TABLE[9].1);
            assert_eq!(TABLE[h.hash("MA")].1, TABLE[20].1);
            assert_eq!(TABLE[h.hash("VA")].1, TABLE[45].1);
            assert_eq!(TABLE[h.hash("KY")].1, TABLE[16].1);
            assert_eq!(TABLE[h.hash("NM")].1, TABLE[30].1);
            assert_eq!(TABLE[h.hash("WA")].1, TABLE[46].1);
            assert_eq!(TABLE[h.hash("UT")].1, TABLE[43].1);
        })
    });
}

fn hashmap_lookup(c: &mut Criterion) {
    let table: HashMap<&str, (&str, u32)> = TABLE
        .iter()
        .map(|&(abbr, state, population)| (abbr, (state, population)))
        .collect();

    c.bench_function("hashmap lookup", move |b| {
        let table = table.clone();
        b.iter(move || {
            assert_eq!(table["NV"].0, TABLE[27].1);
            assert_eq!(table["KS"].0, TABLE[15].1);
            assert_eq!(table["TN"].0, TABLE[41].1);
            assert_eq!(table["GA"].0, TABLE[9].1);
            assert_eq!(table["MA"].0, TABLE[20].1);
            assert_eq!(table["VA"].0, TABLE[45].1);
            assert_eq!(table["KY"].0, TABLE[16].1);
            assert_eq!(table["NM"].0, TABLE[30].1);
            assert_eq!(table["WA"].0, TABLE[46].1);
            assert_eq!(table["UT"].0, TABLE[43].1);
        })
    });
}

criterion_group!(
    benches,
    perfect_hash_lookup,
    hashmap_lookup,
    static_perfect_hash_lookup
);

criterion_main!(benches);
