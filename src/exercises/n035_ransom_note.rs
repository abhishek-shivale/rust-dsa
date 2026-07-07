//! https://leetcode.com/problems/ransom-note/
//!
//! Return whether `ransom_note` can be built using letters from `magazine`,
//! each letter of `magazine` usable at most once.

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_enough_letters() {
        assert!(!can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn enough_letters() {
        assert!(can_construct("aa".to_string(), "aab".to_string()));
    }
}
