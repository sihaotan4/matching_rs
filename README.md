matching_rs is a Rust crate that implements stable matching algorithms.

It implements an ergonomic API for the developing stable matching algorithms.

### To do:
- [x] Implement Gale-Shapley algorithm
- [ ] Implement Irving's algorithm
- [ ] Create preprocessing module for the algorithms
    - [ ] Refine validator to check for invalid inputs (utilize symmetry rules)
    - [ ] Create another validator to check for asymmetric inputs
- [ ] Implement scoring of matching results
    - [ ] Improve Matching struct to deduplicate matches and improve ergonomics with iter methods
- [ ] Check Matching stability with check_stability() method