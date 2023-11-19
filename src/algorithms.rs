use crate::models::{Matches, Rankings};

// Problem is couched to map employees to vacancies
// In this scenario the vacancies (teams) are proposing, employees choose to accept
pub fn gale_shapley(vacancies: Rankings, employees: Rankings) -> Matches {
    let mut matches = Matches::new();

    let mut available_vacancies = vacancies.get_keys();
    let mut vacancies_iter = vacancies.to_iterator_map();

    // while vacancies still remain
    while !available_vacancies.is_empty() {
        let vacancy_to_fill = available_vacancies.remove(0);

        let employee = vacancies_iter.next(&vacancy_to_fill).unwrap();
        // need to handle this unwrap, tho we should not reach this in a gs setup

        // insertion fails then that employee had already been matched
        if let Err(_) = matches.insert(&vacancy_to_fill, &employee) {
            let current_match = matches.get(&employee).unwrap();

            // check if the employee's current match is stable
            if employees.prefers_first(&employee, &current_match, &vacancy_to_fill) {
                // stable, and vacancy remains unfilled
                available_vacancies.push(vacancy_to_fill);

            // else the current match is not stable, change match
            } else {
                // remove existing, insert new, and add the vacancy back into the pool
                matches.remove(&current_match, &employee).unwrap();

                matches.insert(&vacancy_to_fill, &employee).unwrap();

                available_vacancies.push(current_match);
            }
        }
    }
    matches
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

        let matches = gale_shapley(employee_ranking, team_ranking);

        // Check if the matches are stable
        assert_eq!(
            matches.get(&"alice".to_string()), 
            Some("team1".to_string())
        );
        assert_eq!(
            matches.get(&"team2".to_string()),
            Some("charlie".to_string())
        );
        assert_eq!(matches.get(&"bob".to_string()), Some("team3".to_string()));
    }
}
