# chm-hash

Perfect hash function generator based on the CHM algorithm by Z. J. Czech, G. Havas and B.S. Majewski "_An optimal algorithm for generating minimal perfect hash functions_" in Information Processing Letters, 43(5):257-264, 1992.

## Usage

```rust
let animals = ["Elephant", "Horse", "Camel", "Python", "Dog", "Cat"];
let h = generate_hash(
    animals
        .iter()
        .enumerate()
        .map(|(idx, animal)| (animal, idx)), // idx is desired hash value
);
assert!(animals
    .iter()
    .enumerate()
    .all(|(idx, animal)| h.hash(animal) == idx));
```

## WIP

In the above example, you don't want to use the generated version of the hash function dynamically,
but implement it as a static function (cf. [benches/state_lookup.rs](benches/state_lookup.rs)) due
to performance reasons. Just run the benchmarks, you will see why.

* [ ] Provide code generation e.g. by derive macro.
* [ ] Allow to specify seed when generating the hash function, for making tests and benchmarks
  deterministic.

## Acknowledgments

Most of the code is ported from http://ilan.schnell-web.net/prog/perfect-hash/.

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
