//! https://leetcode.com/problems/word-break/
//!
//! Return whether `s` can be segmented into a space-separated sequence of
//! one or more words from `word_dict` (words reusable).

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
    }

    #[test]
    fn example_2() {
        assert!(!word_break(
            "catsandog".to_string(),
            vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]
        ));
    }
}
