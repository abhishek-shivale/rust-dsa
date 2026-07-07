//! https://leetcode.com/problems/permutation-in-string/
//!
//! Return whether `s2` contains a permutation of `s1` as a contiguous
//! substring.

pub fn check_inclusion(s1: String, s2: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_permutation() {
        assert!(check_inclusion("ab".to_string(), "eidbaooo".to_string()));
    }

    #[test]
    fn does_not_contain() {
        assert!(!check_inclusion("ab".to_string(), "eidboaoo".to_string()));
    }
}
