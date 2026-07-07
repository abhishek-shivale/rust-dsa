//! https://leetcode.com/problems/wildcard-matching/
//!
//! Implement wildcard matching for `?` (any single character) and `*`
//! (any sequence, including empty) against the entire string `s`.

pub fn is_match(s: String, p: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_matches_any_sequence() {
        assert!(is_match("adceb".to_string(), "*a*b".to_string()));
    }

    #[test]
    fn no_match() {
        assert!(!is_match("cb".to_string(), "?a".to_string()));
    }
}
