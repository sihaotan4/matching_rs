mod models;
use std::collections::BTreeMap;

use models::Rankings;

mod algorithms;
use algorithms::gale_shapley;

mod checks;

fn main() {
    let proposers = Rankings::from_file("input_data/group_a.txt")
        .expect("Failed to initialize Rankings from file");

    let acceptors = Rankings::from_file("input_data/group_b.txt")
        .expect("Failed to initialize Rankings from file");

    println!("{}", proposers);
    println!("{}", acceptors);

    let matches = gale_shapley(&proposers, &acceptors).unwrap();

    let sorted_matches: BTreeMap<_, _> = matches
        .unique_matches(&proposers, &acceptors, "proposers")
        .into_iter()
        .collect();

    for (key, value) in &sorted_matches {
        println!("{}: {}", key, value);
    }
}

//For these preference lists, there are exactly two stable pairings:
// S = {(1,A), (2,D), (3,C), (4,B)}
// T = {(1,A), (2,C), (3,D), (4,B)}
