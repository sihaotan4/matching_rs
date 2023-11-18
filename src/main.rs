use std::collections::HashMap;
use std::iter::FromIterator;

struct Matching {
    // acting as a bimap, where each match is entered twice
    // first as (x,y) and then as (y,x) to facilitate two-way existence checks
    matches: HashMap<String, String>,
}

impl Matching {
    fn new() -> Matching {
        Matching {
            matches: HashMap::new(),
        }
    }

    fn insert(&mut self, x: String, y: String) -> Result<(), &str> {
        if self.matches.contains_key(&x) || self.matches.contains_key(&y) {
            return Err("Key already present in HashMap");
        }

        self.matches.insert(x.clone(), y.clone());
        self.matches.insert(y, x);
    
        Ok(())
    }

    fn remove(&mut self, x: String, y: String) -> Result<(), &str> {
        if self.matches.contains_key(&x) || self.matches.contains_key(&y) {
            return Err("Key already present in HashMap");
        }

        self.matches.remove(&x);
        self.matches.remove(&y);

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Rankings {
    x_ranking: HashMap<String, Vec<String>>,
    y_ranking: HashMap<String, Vec<String>>,
}

impl Rankings {
    fn from() -> Rankings {
        //build from some kind of input data
        todo!();
    }

    fn to_iterator_map(&self, ranking_type: &str) -> Option<RankingIterMap> {
        // returns a map of iterators for a specified group (x or y)
        match ranking_type {
            "x" => {
                let iter_map = self
                .x_ranking
                .clone()
                .into_iter()
                .map(|(key, values)| (key, values.into_iter()))
                .collect::<RankingIterMap>();
            Some(iter_map)
            }
            "y" => {
                let iter_map = self
                .y_ranking
                .clone()
                .into_iter()
                .map(|(key, values)| (key, values.into_iter()))
                .collect::<RankingIterMap>();
            Some(iter_map)
            }
            _ => None,
        }
    }






    fn remove_next_y(&mut self, x: &String) -> Option<String> {
        // for a given "x", fn returns the next preferred "y"
        // also removes y from x's ranking, to avoid repeating challenges to the same y 
        let x_ranking = self.x_ranking
            .get_mut(x)
            .unwrap();

        if x_ranking.is_empty() {
            return None;
        } else {
            return Some(x_ranking.remove(0));
        }

    }

    // fn create ranking_iter 
    // consumable iter to use the next function which will help with remove_next_y...
    // could be another struct altogether 

    fn check_incumbent_preferred(self, y: &String, challenger_x: &String, incumbent_x: &String) -> bool {
        let ranking = self.y_ranking.get(y).unwrap();


        // get rank should be a separate function entirely
         // Find the index of the element 30
        // if let Some(index) = my_vec.iter().position(|&x| x == 30) {
        //     println!("Index of 30: {}", index);
        // } else {
        //     println!("Element not found");
        // }   
    }

    // solve simple SMP using gale-shapley
    fn solve_smp(self) -> Matching {
        let mut rankings = self.clone();

        let mut matching = Matching::new();

        let mut available_x: Vec<String> = self.x_ranking.keys().map(|s| s.to_owned()).collect();

        while !available_x.is_empty() {
            // multiple remove and pop, consider implementing available_x with DeQue instead of vec
            let challenger_x = available_x.remove(0);

            // todo: what happens if None?
            let y = rankings.remove_next_y(&challenger_x).unwrap();

            if matching.matches.contains_key(&y) {
                let incumbent_x = matching.matches.get(&y).unwrap().to_owned();

                
                

                //     if women_preferences[woman].index(man) < women_preferences[woman].index(current_man):
                //     engaged[woman] = man
                //     men_free.append(current_man)
                // else:
                //     men_free.append(man)

            } else {
                // y is currently not engaged
                matching.insert(challenger_x, y);
            }
        }
        matching
    }

    // fn example_init() -> Rankings {
    //     let x_ranking = vec![
    //         ("x1", vec!["y1", "y2", "y3"]),
    //         ("x2", vec!["y2", "y1", "y3"]),
    //         ("x3", vec!["y1", "y3", "y2"]),
    //     ].into_iter().collect();

    //     let y_ranking = vec![
    //         ("y1", vec!["x2", "x1", "x3"]),
    //         ("y2", vec!["x1", "x2", "x3"]),
    //         ("y3", vec!["x3", "x1", "x2"]),
    //     ].into_iter().collect();

    //     Rankings { x_ranking, y_ranking }
    // }

}

struct RankingIterMap {
    value: HashMap<String, Box<dyn Iterator<Item = String>>>,
}

impl FromIterator<(String, std::vec::IntoIter<String>)> for RankingIterMap {
    fn from_iter<I: IntoIterator<Item=(String, std::vec::IntoIter<String>)>>(iter: I) -> Self {
        let mut iter_map = RankingIterMap {
            value: HashMap::new(),
        };

        for (key, value) in iter {
            iter_map.value.insert(key, Box::new(value));
        }

        iter_map
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn solve_smp() {
        let ranking = Rankings::example_init();
        let mut expected_matching = Matching {
            matches: HashMap::new(),
        };
        expected_matching.matches.insert("x1", "y1");
        expected_matching.matches.insert("x2", "y2");
        expected_matching.matches.insert("x3", "y3");

        let result_matching = ranking.solve_smp();

        assert_eq!(result_matching.matches, expected_matching.matches);
    }
}