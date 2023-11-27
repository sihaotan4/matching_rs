matching_rs is a Rust crate that implements stable matching algorithms.

## Stable matching algorithms

The Gale-Shapley algorithm is implemented for matching two equal sets of participants (e.g., men and women, or employers and job applicants).
- Each participant ranks the members of the opposite set according to their preferences.
- Preferences are complete and transitive (participants can rank all members, and their preferences are consistent).

A more general deferred acceptance algorithm is also implemented.
- The two sets of participants need not be equal.
- Preferences are transitive but need not be complete.

## Warm start scenario

In real-world situations, the user may encounter a warm start scenario where there are already existing matches before running the algorithm. This can create a "musical chair" problem, where a new pairing could replace the original pairing, resulting in new proposers and acceptors entering the pool. 

If the warm start matchings are in play (and coupled proposers are not allowed to make another proposal until they are unseated), the algorithm cannot guarantee a stable matching (this can be proven by contradiction).

To address this:
- Ensure that all proposers and acceptors in the warm start matchings have their preferences recorded in the Rankings.
- Run the deferred acceptance algorithm with the full pool of proposers and acceptors, treating the warm start matchings as if they did not exist.
- Deferred acceptance will resolve this to a stable matching with proposer optimal and acceptor pessimal.

## Building

You'll need a Rust installation in order to compile this. matching_rs compiles with rustc 1.70.0 or newer. 
```
$ git clone https://github.com/sihaotan4/matching_rs.git
$ cd matching_rs
$ cargo run --release
```

## Testing

```
cargo test --all
```

## Convenient data models

`models.rs` contains ergonomic data structures to easily develop and modify stable matching algorithms. This allows for easy and intuitive development of other matching algorithms. 
- The `Matching` struct is implemented as a bimap which allows for fast look up on both `k` and `v`.
- `Rankings` is a convenient struct to store the participants ordinal rankings.
- `RankingIterMap` is conveniently derived from `Rankings`. `RankingIterMap` allows for storage and retrieval of iterators associated with specific particpants. This removes the need to keep track of which rankings have been exhausted as the algorithm runs. For e.g. the `next()` method for this struct takes a key and advances the iterator only for that key.

### To do:
- [ ] Refine validator to check for invalid inputs (utilize symmetry rules)
- [ ] Create another validator to check for asymmetric inputs
- [ ] Implement scoring of matching results
- [ ] Check Matching stability with check_stability() method
