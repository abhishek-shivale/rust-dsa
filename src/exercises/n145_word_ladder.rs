//! https://leetcode.com/problems/word-ladder/
//!
//! Each step changes one letter to reach a word in `word_list`. Return the
//! shortest transformation length from `begin_word` to `end_word`
//! (inclusive of both ends), or 0 if no path exists.

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ws(s: &[&str]) -> Vec<String> {
        s.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn example_1() {
        let words = ws(&["hot", "dot", "dog", "lot", "log", "cog"]);
        assert_eq!(ladder_length("hit".to_string(), "cog".to_string(), words), 5);
    }

    #[test]
    fn end_word_missing() {
        let words = ws(&["hot", "dot", "dog", "lot", "log"]);
        assert_eq!(ladder_length("hit".to_string(), "cog".to_string(), words), 0);
    }
}
