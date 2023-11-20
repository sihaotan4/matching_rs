mod models;
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

    println!(
        "{:?}",
        matches.unique_matches(&proposers, &acceptors, "acceptors")
    );
}

//For these preference lists, there are exactly two stable pairings:
// S = {(1,A), (2,D), (3,C), (4,B)}
// T = {(1,A), (2,C), (3,D), (4,B)}
