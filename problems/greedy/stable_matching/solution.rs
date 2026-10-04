use std::collections::{HashMap, VecDeque};

fn initialize_men(
    men_preferences: &HashMap<String, Vec<String>>,
) -> (VecDeque<String>, HashMap<String, usize>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (initialize_men)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement initialize_men");
}

fn initialize_women(
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, Option<String>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (initialize_women)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement initialize_women");
}

fn precompute_woman_ranks(
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, usize>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (precompute_woman_ranks)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement precompute_woman_ranks");
}

fn process_proposal(
    man: &str,
    free_men: &mut VecDeque<String>,
    current_partner: &mut HashMap<String, Option<String>>,
    man_engaged: &mut HashMap<String, Option<String>>,
    next_proposal: &mut HashMap<String, usize>,
    men_preferences: &HashMap<String, Vec<String>>,
    woman_ranks: &HashMap<String, HashMap<String, usize>>,
) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (process_proposal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement process_proposal");
}

fn woman_prefers_new_man(
    woman: &str,
    man1: &str,
    man2: &str,
    woman_ranks: &HashMap<String, HashMap<String, usize>>,
) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (woman_prefers_new_man)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement woman_prefers_new_man");
}

fn engage_man(
    man: &str,
    woman: &str,
    free_men: &mut VecDeque<String>,
    current_partner: &mut HashMap<String, Option<String>>,
    man_engaged: &mut HashMap<String, Option<String>>,
    current_man: Option<String>,
) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (engage_man)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement engage_man");
}

fn finalize_matches(man_engaged: HashMap<String, Option<String>>) -> HashMap<String, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (finalize_matches)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement finalize_matches");
}

pub fn stable_matching(
    men_preferences: &HashMap<String, Vec<String>>,
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (stable_matching)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement stable_matching");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_stable_matching_scenario_1() {
        let men_preferences = HashMap::from([
            (
                "A".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "B".to_string(),
                vec!["Y".to_string(), "X".to_string(), "Z".to_string()],
            ),
            (
                "C".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
        ]);

        let women_preferences = HashMap::from([
            (
                "X".to_string(),
                vec!["B".to_string(), "A".to_string(), "C".to_string()],
            ),
            (
                "Y".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
            (
                "Z".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);

        let expected_matches1 = HashMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        let expected_matches2 = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        assert!(matches == expected_matches1 || matches == expected_matches2);
    }

    #[test]
    fn test_stable_matching_empty() {
        let men_preferences = HashMap::new();
        let women_preferences = HashMap::new();

        let matches = stable_matching(&men_preferences, &women_preferences);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_stable_matching_duplicate_preferences() {
        let men_preferences = HashMap::from([
            ("A".to_string(), vec!["X".to_string(), "X".to_string()]), // Man with duplicate preferences
            ("B".to_string(), vec!["Y".to_string()]),
        ]);

        let women_preferences = HashMap::from([
            ("X".to_string(), vec!["A".to_string(), "B".to_string()]),
            ("Y".to_string(), vec!["B".to_string()]),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);
        let expected_matches = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
        ]);

        assert_eq!(matches, expected_matches);
    }

    #[test]
    fn test_stable_matching_single_pair() {
        let men_preferences = HashMap::from([("A".to_string(), vec!["X".to_string()])]);
        let women_preferences = HashMap::from([("X".to_string(), vec!["A".to_string()])]);

        let matches = stable_matching(&men_preferences, &women_preferences);
        let expected_matches = HashMap::from([("A".to_string(), "X".to_string())]);

        assert_eq!(matches, expected_matches);
    }
    #[test]
    fn test_woman_prefers_new_man() {
        let men_preferences = HashMap::from([
            (
                "A".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "B".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "C".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
        ]);

        let women_preferences = HashMap::from([
            (
                "X".to_string(),
                vec!["B".to_string(), "A".to_string(), "C".to_string()],
            ),
            (
                "Y".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
            (
                "Z".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);

        let expected_matches = HashMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        assert_eq!(matches, expected_matches);
    }
}
