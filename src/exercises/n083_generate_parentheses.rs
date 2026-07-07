//! https://leetcode.com/problems/generate-parentheses/
//!
//! Return all combinations of `n` pairs of well-formed parentheses.

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn three_pairs() {
        let result: HashSet<String> = generate_parenthesis(3).into_iter().collect();
        let expected: HashSet<String> = vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(result, expected);
    }
}
