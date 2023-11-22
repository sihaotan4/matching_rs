matching_rs is a Rust crate that implements stable matching algorithms.

## Stable matching algorithms

The Gale-Shapley algorithm is implemented for matching two equal sets of participants (e.g., men and women, or employers and job applicants).
- Each participant ranks the members of the opposite set according to their preferences.
- Preferences are complete and transitive (participants can rank all members, and their preferences are consistent).

A more general deferred acceptance algorithm is also implemented.
- The two sets of participants need not be equal.
- Preferences are transitive but need not be complete.

Future work - to implement a version of the deferred acceptance algorithm with a **warm start**. This is a _musical chair_ scenario when there are already some Matches before the algorithm begins. At runtime, these starting matches may be broken, releasing new participants into the proposal pool. This mimics a real scenario in an organisation with job seekers and job vacancies. In this scenario, internal rotations are possible, so an employee leaving an existing job (in an exiting match) also creates a job vacancy when he enters a new match.  

## convenient data models

`models.rs` contains ergonomic data structures to easily develop and modify stable matching algorithms. This allows for easy and intuitive development of other matching algorithms. 
- The `Matching` struct is implemented as a bimap which allows for fast look up on both `k` and `v`.
- `Rankings` is a convenient struct to store the participants ordinal rankings.
- `RankingIterMap` is conveniently derived from `Rankings`. `RankingIterMap` allows for storage and retrieval of iterators associated with specific particpants. This removes the need to keep track of which rankings have been exhausted as the algorithm runs. For e.g. the `next()` method for this struct takes a key and advances the iterator only for that key.


### To do:
- [ ] Refine validator to check for invalid inputs (utilize symmetry rules)
- [ ] Create another validator to check for asymmetric inputs
- [ ] Implement scoring of matching results
- [ ] Check Matching stability with check_stability() method
