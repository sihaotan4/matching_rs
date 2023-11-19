mod models;
use models::Rankings;

mod algorithms;
use algorithms::gale_shapley;

fn main() {
    // ingest both datasets, e.g. one for teams' ranking of preferred employees, 
    // another for employees' ranking of preferred teams 
    let employee_ranking  = Rankings::from_file("test_data/gs_test_employees.txt")
            .expect("Failed to initialize Rankings");

    let team_ranking  = Rankings::from_file("test_data/gs_test_vacancies.txt")
            .expect("Failed to initialize Rankings");

    println!("{}", employee_ranking);
    println!("{}", team_ranking);

    let matches = gale_shapley(employee_ranking, team_ranking);

    println!("{}", matches);

}