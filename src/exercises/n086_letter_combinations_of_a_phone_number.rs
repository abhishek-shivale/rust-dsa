//! https://leetcode.com/problems/letter-combinations-of-a-phone-number/
//!
//! Return every letter combination that `digits` (2-9) could represent on
//! a phone keypad, in any order.

pub fn letter_combinations(digits: String) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_1() {
        let result: HashSet<String> = letter_combinations("23".to_string()).into_iter().collect();
        let expected: HashSet<String> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_input() {
        assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());
    }
}
