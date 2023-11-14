use std::collections::HashMap;

#[derive(Debug)]
struct Rankings {
    x: HashMap<&'static str, Vec<&'static str>>,
    y: HashMap<&'static str, Vec<&'static str>>,
}

impl Rankings {
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

        Rankings { x: x_ranking, y: y_ranking }
    }

    fn build_from() -> Rankings {
        //build from some kind of input data
        todo!();
    }

    // solve simple SMP using Gale-Shapley
    fn solve_smp() -> (usize, usize) {
        todo!();
    }

}

fn main() {

    let ranking = Rankings::example_init(); 

    println!("{:?}", ranking);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solve_smp() {
        let ranking = Rankings::example_init();

        

    }
}