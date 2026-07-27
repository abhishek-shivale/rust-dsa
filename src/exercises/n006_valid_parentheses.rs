//! https://leetcode.com/problems/valid-parentheses/
//!
//! `s` contains only `()[]{}`. Return whether every bracket is closed in
//! the correct order.

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for bracket in s.chars() {
        match bracket {
            '(' | '[' | '{' => stack.push(bracket),
            ')' if stack.pop() == Some('(') => {}
            ']' if stack.pop() == Some('[') => {}
            '}' if stack.pop() == Some('{') => {}
            _ => return false,
        }
    }

    stack.is_empty()
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
        assert!(!is_valid("]".to_string()));
        assert!(!is_valid("([)]".to_string()));
        assert!(!is_valid("(".to_string()));
    }
}
