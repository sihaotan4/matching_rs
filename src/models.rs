use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, Read};

/**
 * Rankings stores the preferences of one type of participants in the context of a stable matching problem.
 * For two party matching, two Rankings structs are needed to represent the preferences of both
 * participant types.
 */
#[derive(Debug)]
pub struct Rankings {
    map: HashMap<String, Vec<String>>,
}

impl fmt::Display for Rankings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n")?;
        for (key, value) in &self.map {
            write!(f, "    {}: {:?},\n", key, value)?;
        }
        write!(f, "}}")
    }
}

impl Rankings {
    fn from_str(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();

        let line_regex = Regex::new(r#"(?m)^(.+?),(.+)$"#)?;

        for capture in line_regex.captures_iter(input) {
            let name = capture[1].to_string();
            let values = capture[2]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            // if this errors, it means the input data is incorrect
            if map.insert(name.clone(), values).is_some() {
                return Err(format!("Data input error: Duplicate key: {}", name).into());
            }
        }

        Ok(Rankings { map })
    }

    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // this just panics
        let file_content = read_file(file_path).unwrap();

        // this actually propagates regex::Error, todo: custom RankingError enum
        Ok(Rankings::from_str(&file_content)?)
    }

    pub fn get_rank(&self, participant: &String, target: &String) -> Option<usize> {
        if let Some(x) = self.map.get(participant) {
            x.into_iter().position(|x| x == target)
        } else {
            None
        }
    }

    pub fn prefers_first(&self, participant: &String, first: &String, second: &String) -> bool {
        let first_rank = self.get_rank(participant, first).unwrap();
        let second_rank = self.get_rank(participant, second).unwrap();

        if first_rank < second_rank {
            true
        } else if first_rank > second_rank {
            false
        } else {
            panic!("Equal ranks should not be possible")
        }
    }

    pub fn to_iterator_map(&self) -> RankingIterMap {
        let iter_map = self
            .map
            .clone()
            .into_iter()
            .map(|(key, values)| (key, values.into_iter()))
            .collect::<RankingIterMap>();

        iter_map
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect::<Vec<String>>()
    }
}

pub struct RankingIterMap {
    map: HashMap<String, Box<dyn Iterator<Item = String>>>,
}

impl FromIterator<(String, std::vec::IntoIter<String>)> for RankingIterMap {
    fn from_iter<I: IntoIterator<Item = (String, std::vec::IntoIter<String>)>>(iter: I) -> Self {
        let mut iter_map = RankingIterMap {
            map: HashMap::new(),
        };

        for (key, value) in iter {
            iter_map.map.insert(key, Box::new(value));
        }

        iter_map
    }
}
impl RankingIterMap {
    pub fn next(&mut self, participant: &String) -> Option<String> {
        self.map.get_mut(participant).and_then(|iter| iter.next())
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file_content = String::new();
    File::open(file_path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn validate_rankings(rankings1: &Rankings, rankings2: &Rankings) -> Result<(), &'static str> {
    let keys1: HashSet<_> = rankings1.map.keys().collect();
    let keys2: HashSet<_> = rankings2.map.keys().collect();

    // Check that the set of preferences is exactly the same as the keys in the other Ranking
    for (_key, preferences) in &rankings1.map {
        let preferences_set: HashSet<_> = preferences.iter().collect();
        if preferences_set != keys2 {
            return Err("Data input error: Preferences in ranking1 are not the same as the entities in ranking2");
        }
    }

    for (_key, preferences) in &rankings2.map {
        let preferences_set: HashSet<_> = preferences.iter().collect();
        if preferences_set != keys1 {
            return Err("Data input error: Preferences in ranking2 are not the same as the entities in ranking1");
        }
    }

    // Check that all preferences are listed
    for (_key, preferences) in &rankings1.map {
        if preferences.len() != keys2.len() {
            return Err("Data input error: Number of preferences does not equal number of entities listed in ranking1");
        }
    }

    for (_key, preferences) in &rankings2.map {
        if preferences.len() != keys1.len() {
            return Err("Data input error: Number of preferences does not equal number of entities listed in ranking1");
        }
    }

    Ok(())
}

/**
 * Matches implements a simple bimap.
 * Each match is entered twice. First as (x,y) and then again as (y,x) to facilitate two-way existence checks
 */
#[derive(Debug)]
pub struct Matches {
    map: HashMap<String, String>,
}

impl fmt::Display for Matches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n")?;
        for (key, value) in &self.map {
            write!(f, "    {}: {},\n", key, value)?;
        }
        write!(f, "}}")
    }
}

impl Matches {
    pub fn new() -> Matches {
        Matches {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, x: &String, y: &String) -> Result<(), &str> {
        if self.map.contains_key(x) || self.map.contains_key(y) {
            return Err("Key already present in Matches");
        }

        self.map.insert(x.clone(), y.clone());
        self.map.insert(y.clone(), x.clone());

        Ok(())
    }

    pub fn remove(&mut self, x: &String, y: &String) -> Result<(), &str> {
        if !self.map.contains_key(x) || !self.map.contains_key(y) {
            return Err("Match not present in Matches. Cannot remove.");
        }

        self.map.remove(x);
        self.map.remove(y);

        Ok(())
    }

    pub fn get(&self, key: &String) -> Option<String> {
        self.map.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    // Lazy static variable to hold Rankings instance
    lazy_static! {
        static ref TEST_RANKING: Rankings = Rankings::from_file("test_data/unit_test_a.txt")
            .expect("Failed to initialize Rankings for tests");
    }

    #[test]
    fn test_ranking_get_rank() {
        let test_cases = vec![
            ("alice", "team1", Some(0_usize)),
            ("bob", "team3", Some(1_usize)),
            ("charlie", "team3", Some(2_usize)),
        ];

        for case in test_cases {
            assert_eq!(
                TEST_RANKING.get_rank(&case.0.to_string(), &case.1.to_string()),
                case.2
            );
        }
    }

    #[test]
    fn test_ranking_prefers_first() {
        let test_cases = vec![
            // participant, first target, second target, expected result
            ("alice", "team1", "team3", true),
            ("bob", "team3", "team1", false),
            ("charlie", "team2", "team1", true),
        ];

        for (participant, first, second, expected) in test_cases {
            let result = TEST_RANKING.prefers_first(
                &participant.to_string(),
                &first.to_string(),
                &second.to_string(),
            );
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_iter_map_next() {
        let mut iter_map = TEST_RANKING.to_iterator_map();

        let test_cases = vec![
            ("alice", Some("team1".to_string())),
            ("alice", Some("team2".to_string())),
            ("bob", Some("team1".to_string())),
            ("charlie", Some("team2".to_string())),
            ("bob", Some("team3".to_string())),
            ("alice", Some("team3".to_string())),
            ("alice", None),
        ];

        for case in test_cases {
            assert_eq!(iter_map.next(&case.0.to_string()), case.1);
        }
    }

    #[test]
    fn test_insert_matches() {
        let mut matches = Matches::new();

        // Test inserting a new match
        assert_eq!(
            matches.insert(&"Participant1".to_string(), &"Target1".to_string()),
            Ok(())
        );
        assert_eq!(
            matches.get(&"Participant1".to_string()),
            Some("Target1".to_string())
        );
        assert_eq!(
            matches.get(&"Target1".to_string()),
            Some("Participant1".to_string())
        );

        // Test inserting a match with a participant that already exists
        assert_eq!(
            matches.insert(&"Participant1".to_string(), &"Target2".to_string()),
            Err("Key already present in Matches")
        );

        // Test inserting a match with a target that already exists
        assert_eq!(
            matches.insert(&"Participant2".to_string(), &"Target1".to_string()),
            Err("Key already present in Matches")
        );
    }

    #[test]
    fn test_remove_matches() {
        let mut matches = Matches::new();

        // Insert a match
        matches
            .insert(&"Participant1".to_string(), &"Target1".to_string())
            .unwrap();

        // Test removing a match that exists
        assert_eq!(
            matches.remove(&"Participant1".to_string(), &"Target1".to_string()),
            Ok(())
        );
        assert_eq!(matches.map.get("Participant1"), None);
        assert_eq!(matches.map.get("Target1"), None);

        // Test removing a match that does not exist
        assert_eq!(
            matches.remove(&"Participant1".to_string(), &"Target1".to_string()),
            Err("Match not present in Matches. Cannot remove.")
        );

        // Insert a match
        matches
            .insert(&"Participant1".to_string(), &"Target3".to_string())
            .unwrap();

        // Test removing a match that partially exists
        assert_eq!(
            matches.remove(&"Participant1".to_string(), &"Target2".to_string()),
            Err("Match not present in Matches. Cannot remove.")
        );

        // Finally remove it
        assert_eq!(
            matches.remove(&"Participant1".to_string(), &"Target3".to_string()),
            Ok(())
        );
    }
}
