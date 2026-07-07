//! https://leetcode.com/problems/valid-sudoku/
//!
//! Return whether the partially-filled 9x9 `board` (`'.'` for empty) is
//! valid so far: no row, column, or 3x3 box repeats a digit.

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn valid() {
        let b = board(&[
            "53..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1",
            "7...2...6", ".6....28.", "...419..5", "....8..79",
        ]);
        assert!(is_valid_sudoku(b));
    }

    #[test]
    fn invalid_duplicate() {
        let b = board(&[
            "83..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1",
            "7...2...6", ".6....28.", "...419..5", "....8..79",
        ]);
        assert!(!is_valid_sudoku(b));
    }
}
