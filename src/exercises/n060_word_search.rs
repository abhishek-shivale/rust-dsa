//! https://leetcode.com/problems/word-search/
//!
//! Return whether `word` can be traced through adjacent (4-directional)
//! cells of `board`, each cell used at most once.

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn found() {
        let b = board(&["ABCE", "SFCS", "ADEE"]);
        assert!(exist(b, "ABCCED".to_string()));
    }

    #[test]
    fn not_found() {
        let b = board(&["ABCE", "SFCS", "ADEE"]);
        assert!(!exist(b, "ABCB".to_string()));
    }
}
