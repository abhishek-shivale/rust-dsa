//! https://leetcode.com/problems/valid-parentheses/
//!
//! `s` contains only `()[]{}`. Return whether every bracket is closed in
//! the correct order.

pub fn is_valid(s: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn invalid() {
        assert!(!is_valid("(]".to_string()));
    }
}
