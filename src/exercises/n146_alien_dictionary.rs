//! https://leetcode.com/problems/alien-dictionary/ (Premium)
//!
//! `words` are sorted according to some unknown alien alphabet. Return one
//! valid ordering of that alphabet's letters, or `""` if the ordering is
//! invalid/ambiguous in a way that admits no valid string (impossible
//! constraints); for a fully ambiguous-but-consistent case, any valid
//! topological order is accepted, so tests only check invalid input here.

pub fn alien_order(words: Vec<String>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_order() {
        let words = vec!["z".to_string(), "x".to_string(), "z".to_string()];
        assert_eq!(alien_order(words), "");
    }

    #[test]
    fn single_word() {
        assert_eq!(alien_order(vec!["abc".to_string()]), "abc".to_string());
    }
}
