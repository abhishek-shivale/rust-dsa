//! https://leetcode.com/problems/word-search-ii/
//!
//! Return every word from `words` that can be traced through adjacent
//! (4-directional) cells of `board`, each cell used at most once per word.

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn board(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn example_1() {
        let b = board(&["oaan", "etae", "ihkr", "iflv"]);
        let words = vec!["oath", "pea", "eat", "rain"].into_iter().map(String::from).collect();
        let result: HashSet<String> = find_words(b, words).into_iter().collect();
        assert_eq!(result, HashSet::from(["oath".to_string(), "eat".to_string()]));
    }
}
