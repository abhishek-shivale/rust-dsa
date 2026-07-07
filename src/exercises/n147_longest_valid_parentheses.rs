//! https://leetcode.com/problems/longest-valid-parentheses/
//!
//! `s` contains only `(` and `)`. Return the length of the longest
//! substring that is well-formed.

pub fn longest_valid_parentheses(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    }
}
