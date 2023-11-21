use crate::checks::check_ranking_symmetry;
use crate::models::{Matches, Rankings};

pub fn run_gale_shapley(proposers: &Rankings, acceptors: &Rankings) -> Result<Matches, &'static str> {
    // checks if both Ranking structs are set up for gale_shapley assumptions
    check_ranking_symmetry(proposers, acceptors)?;

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
        if matches.insert(&proposer, &acceptor).is_err() {
            // find the proposer that is currently matched with this acceptor
            let incumbent_proposer = matches.get(&acceptor).unwrap();

            // check if the current match is stable
            // in a clean symmetrical problem this will not return None
            if acceptors.prefers_first(&acceptor, &incumbent_proposer, &proposer).unwrap() {
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

pub fn run_deferred_acceptance(proposers: &Rankings, acceptors: &Rankings) -> Result<Matches, &'static str> {
    // todo! symmetry not required but input checks are impt
    // check_ranking_symmetry(proposers, acceptors)?;

    let mut matches = Matches::new();

    let mut available_proposers = proposers.get_keys();
    let mut proposers_iter = proposers.to_iterator_map();

    // while the pool of proposers still remain
    while !available_proposers.is_empty() {
        let proposer = available_proposers.remove(0);

        // get the proposer's next preferred acceptor
        let acceptor = match proposers_iter.next(&proposer) {
            Some(acceptor) => acceptor,
            None => {
                // if None, do not push him back into available_proposers,
                continue;
            },
        };

        // check if the acceptor even has a preference for the proposer
        if acceptors.get_rank(&acceptor, &proposer).is_none() {
            // do not insert match and just return the proposed to the pool
            available_proposers.push(proposer);
            continue;
        }

        // check if insertion fails, meaning that this acceptor had already been matched
        if matches.insert(&proposer, &acceptor).is_err() {
            // find the proposer that is currently matched with this acceptor
            let incumbent_proposer = matches.get(&acceptor).unwrap();

            // check if the current match is stable
            match acceptors.prefers_first(&acceptor, &incumbent_proposer, &proposer) {
                Some(true) => {
                    // current match is stable, the proposer is unsuccessful and returns to the pool
                    available_proposers.push(proposer);
                },
                Some(false) => {
                    // current match is unstable, and they can trade up
                    matches.remove(&incumbent_proposer, &acceptor).unwrap();
                    matches.insert(&proposer, &acceptor).unwrap();

                    // incumbent returns to the pool
                    available_proposers.push(incumbent_proposer);
                },
                None => {
                    // the acceptor has no declared ranking for our proposer at all and returns to the pool
                    // todo this branch should not be reached because we check get_rank already
                    available_proposers.push(proposer);
                },
            };

        }
        else {
            // match insert succeeds, continue
            continue;
        }
        
    }
    Ok(matches)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gale_shapley() {
        let proposers = Rankings::from_file("test_data/unit_test_a.txt")
            .expect("Failed to initialize Rankings");

        let acceptors = Rankings::from_file("test_data/unit_test_b.txt")
            .expect("Failed to initialize Rankings");

        let matches = run_gale_shapley(&proposers, &acceptors).unwrap();

        // Check if the matches are stable
        assert_eq!(matches.get(&"alice".to_string()), Some("team1".to_string()));
        assert_eq!(
            matches.get(&"team2".to_string()),
            Some("charlie".to_string())
        );
        assert_eq!(matches.get(&"bob".to_string()), Some("team3".to_string()));
    }
}
