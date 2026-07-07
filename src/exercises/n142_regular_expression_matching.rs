//! https://leetcode.com/problems/regular-expression-matching/
//!
//! Implement regex matching for `.` (any character) and `*` (zero or more
//! of the preceding element) against the entire string `s`.

pub fn is_match(s: String, p: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_matches_zero() {
        assert!(is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn dot_star_matches_any() {
        assert!(is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn no_match() {
        assert!(!is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
    }
}
