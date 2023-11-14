use std::collections::HashMap;

struct Matching {
    //matches: Vec<(&'static str, &'static str)>
    matches: HashMap<&'static str, &'static str>,
}

#[derive(Debug)]
struct Rankings {
    x_ranking: HashMap<&'static str, Vec<&'static str>>,
    y_ranking: HashMap<&'static str, Vec<&'static str>>,
}

impl Rankings {
    fn build_from() -> Rankings {
        //build from some kind of input data
        todo!();
    }

    // solve simple SMP using gale-shapley
    fn solve_smp(&self) -> Matching {

        // clone a mutable copy of self
        let mut rankings = self.clone();

        // mutable list of available x
        let mut x_available: Vec<&str> = self.x_ranking.keys().map(|s| *s).collect();

        let mut matching = Matching {
            matches: HashMap::new(),
        };

        // while x_available is not empty, continue
        while !x_available.is_empty() {
            let x = x_available.pop().unwrap();

            // get x next best target
            // deal with unwrap
            // deal with remove being more costly than pop
            let y = rankings.x_ranking.get(x).unwrap().remove(0);

            // matches is key-value of y,x
            // check if y is not already matched
            if !matching.matches.contains_key(y) {
                matching.matches.insert(y, x);
            
            // compete with current rival
            } else {
                let rival = *matching.matches.get(y).unwrap();

            //     if women_preferences[woman].index(man) < women_preferences[woman].index(current_man):
            //     engaged[woman] = man
            //     men_free.append(current_man)
            // else:
            //     men_free.append(man)

            }

        Matching {
            matches: HashMap::new(),
        }
        
    }

    // initialize Rankings with example data
    fn example_init() -> Rankings {
        let x_ranking = vec![
            ("x1", vec!["y1", "y2", "y3"]),
            ("x2", vec!["y2", "y1", "y3"]),
            ("x3", vec!["y1", "y3", "y2"]),
        ].into_iter().collect();

        let y_ranking = vec![
            ("y1", vec!["x2", "x1", "x3"]),
            ("y2", vec!["x1", "x2", "x3"]),
            ("y3", vec!["x3", "x1", "x2"]),
        ].into_iter().collect();

        Rankings { x_ranking: x_ranking, y_ranking: y_ranking }
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