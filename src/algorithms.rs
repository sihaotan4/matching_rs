use crate::models::{validate_rankings, Matches, Rankings};

pub fn gale_shapley(proposers: Rankings, acceptors: Rankings) -> Result<Matches, &'static str> {
    // checks if both Ranking structs are set up for gale_shapley assumptions
    validate_rankings(&proposers, &acceptors)?;

    let mut matches = Matches::new();

    let mut available_proposers = proposers.get_keys();
    let mut proposers_iter = proposers.to_iterator_map();

    // while the pool of proposers still remain
    while !available_proposers.is_empty() {
        let proposer = available_proposers.remove(0);

        // get his next preferred acceptor
        // need to handle this unwrap, tho we should not reach this in a gs setup
        let acceptor = proposers_iter.next(&proposer).unwrap();

        // check if insertion fails, if failed then that acceptor had already been matched
        if let Err(_) = matches.insert(&proposer, &acceptor) {
            // find the proposer that is currently matched with this acceptor
            let incumbent_proposer = matches.get(&acceptor).unwrap();

            // check if the current match is stable
            if acceptors.prefers_first(&acceptor, &incumbent_proposer, &proposer) {
                // if stable, the proposal challenge is unsuccessful and they return to the pool
                available_proposers.push(proposer);

            // else the current match is not stable, change match
            } else {
                // swap the matches
                matches.remove(&incumbent_proposer, &acceptor).unwrap();
                matches.insert(&proposer, &acceptor).unwrap();

                // incumbent returns to the pool
                available_proposers.push(incumbent_proposer);
            }
        }
    }
    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gale_shapley() {
        let employee_ranking = Rankings::from_file("test_data/unit_test_a.txt")
            .expect("Failed to initialize Rankings");

        let team_ranking = Rankings::from_file("test_data/unit_test_b.txt")
            .expect("Failed to initialize Rankings");

        let matches = gale_shapley(employee_ranking, team_ranking).unwrap();

        // Check if the matches are stable
        assert_eq!(matches.get(&"alice".to_string()), Some("team1".to_string()));
        assert_eq!(
            matches.get(&"team2".to_string()),
            Some("charlie".to_string())
        );
        assert_eq!(matches.get(&"bob".to_string()), Some("team3".to_string()));
    }
}