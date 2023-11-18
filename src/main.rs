mod models;

pub use crate::models::{Matches, Rankings, RankingIterMap, read_file};

fn main() {
    let employee_ranking  = Rankings::from_file("test_data/test1.csv")
            .expect("Failed to initialize Rankings for tests");

    println!("{:?}", employee_ranking);

}


    // solve simple SMP using gale-shapley
    // fn solve_smp(self) -> Matching {
    //     let mut rankings = self.clone();

    //     let mut matching = Matching::new();

    //     let mut available_x: Vec<String> = self.x_ranking.keys().map(|s| s.to_owned()).collect();

    //     while !available_x.is_empty() {
    //         // multiple remove and pop, consider implementing available_x with DeQue instead of vec
    //         let challenger_x = available_x.remove(0);

    //         // todo: what happens if None?
    //         let y = rankings.remove_next_y(&challenger_x).unwrap();

    //         if matching.matches.contains_key(&y) {
    //             let incumbent_x = matching.matches.get(&y).unwrap().to_owned();

                

    //             //     if women_preferences[woman].index(man) < women_preferences[woman].index(current_man):
    //             //     engaged[woman] = man
    //             //     men_free.append(current_man)
    //             // else:
    //             //     men_free.append(man)

    //         } else {
    //             // y is currently not engaged
    //             matching.insert(challenger_x, y);
    //         }
    //     }
    //     matching
    

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
