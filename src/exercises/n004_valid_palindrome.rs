//! https://leetcode.com/problems/valid-palindrome/
//!
//! Lowercase `s`, strip non-alphanumerics, return whether it reads the same
//! forwards and backwards.

pub fn is_palindrome(s: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn not_palindrome() {
        assert!(!is_palindrome("race a car".to_string()));
    }
}
