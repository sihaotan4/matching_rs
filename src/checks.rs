use std::collections::HashSet;

use crate::models::Rankings;

pub fn check_ranking_symmetry(
    rankings1: &Rankings,
    rankings2: &Rankings,
) -> Result<(), &'static str> {
    let binding = rankings1.get_keys();
    let keys1: HashSet<&String> = binding.iter().collect();

    let binding = rankings2.get_keys();
    let keys2: HashSet<&String> = binding.iter().collect();

    // Check that the set of preferences is exactly the same as the keys in the other Ranking
    for preferences in rankings1.get_map().values() {
        let preferences_set: HashSet<_> = preferences.iter().collect();
        if preferences_set != keys2 {
            return Err("Data input error: Preferences in ranking1 are not the same as the entities in ranking2");
        }
    }

    for preferences in rankings2.get_map().values() {
        let preferences_set: HashSet<_> = preferences.iter().collect();
        if preferences_set != keys1 {
            return Err("Data input error: Preferences in ranking2 are not the same as the entities in ranking1");
        }
    }

    // Checks for duplicate data, resulting in a longer list of preferences
    for preferences in rankings1.get_map().values() {
        if preferences.len() != keys2.len() {
            return Err("Data input error: Number of preferences does not equal number of entities listed in ranking1");
        }
    }

    for preferences in rankings2.get_map().values() {
        if preferences.len() != keys1.len() {
            return Err("Data input error: Number of preferences does not equal number of entities listed in ranking1");
        }
    }

    Ok(())
}
