//! https://leetcode.com/problems/valid-anagram/
//!
//! Return whether `t` is an anagram of `s` (same letters, same counts).

pub fn is_anagram(s: String, t: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn not_anagram() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
