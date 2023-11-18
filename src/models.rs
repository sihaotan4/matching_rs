use std::collections::HashMap;
use std::fs::File;
use std::fmt;
use std::io::{self, Read};
use regex::Regex;

/**
 * Rankings stores the preferences of one type of participants in the context of a stable matching problem.
 * For two party matching, two Rankings structs are needed to represent the preferences of both 
 * participant types.
 */
#[derive(Debug)]
pub struct Rankings {
    map: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub enum RankingsError {
    IoError(io::Error),
    ParsingError(&'static str),
}

impl fmt::Display for RankingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RankingsError::IoError(err) => write!(f, "IO Error: {}", err),
            RankingsError::ParsingError(err) => write!(f, "Parsing Error: {}", err),
        }
    }
}

impl Rankings {
    fn from_str(input: &str) -> Result<Self, RankingsError> {
        let mut map = HashMap::new();

        let line_regex = Regex::new(r#"(?m)^(.+?),(.+)$"#).unwrap();

        for capture in line_regex.captures_iter(input) {
            let name = capture[1].to_string();
            let values = capture[2].split(',').map(|s| s.trim().to_string()).collect();

            map.insert(name, values);
        }

        Ok(Rankings { map })
    }

    pub fn from_file(file_path: &str) -> Result<Self, RankingsError> {
        let file_content = read_file(file_path).map_err(RankingsError::IoError)?;
        Rankings::from_str(&file_content).map_err(|e| RankingsError::ParsingError(&e.to_string()))
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

}

pub struct RankingIterMap {
    map: HashMap<String, Box<dyn Iterator<Item = String>>>,
}

impl FromIterator<(String, std::vec::IntoIter<String>)> for RankingIterMap {
    fn from_iter<I: IntoIterator<Item=(String, std::vec::IntoIter<String>)>>(iter: I) -> Self {
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
        self.map
            .get_mut(participant)
            .and_then(|iter| iter.next())
    }
}

/**
 * Matches implements a simple bimap.
 * Each match is entered twice. First as (x,y) and then again as (y,x) to facilitate two-way existence checks
 */
pub struct Matches {
    map: HashMap<String, String>,
}

impl Matches {
    fn new() -> Matches {
        Matches {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, x: String, y: String) -> Result<(), &str> {
        if self.map.contains_key(&x) || self.map.contains_key(&y) {
            return Err("Key already present in HashMap");
        }

        self.map.insert(x.clone(), y.clone());
        self.map.insert(y, x);
    
        Ok(())
    }

    fn remove(&mut self, x: String, y: String) -> Result<(), &str> {
        if self.map.contains_key(&x) || self.map.contains_key(&y) {
            return Err("Key already present in HashMap");
        }

        self.map.remove(&x);
        self.map.remove(&y);

        Ok(())
    }
}

pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file_content = String::new();
    File::open(file_path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

     // Lazy static variable to hold Rankings instance
     lazy_static! {
        static ref EMPLOYEE_RANKING: Rankings = Rankings::from_file("test_data/test1.csv")
            .expect("Failed to initialize Rankings for tests");
    }


    #[test]
    fn solve_smp() {

        assert_eq!(true, true);
    }
}

