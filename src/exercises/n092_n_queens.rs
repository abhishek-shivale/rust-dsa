//! https://leetcode.com/problems/n-queens/
//!
//! Return the number of distinct solutions to placing `n` non-attacking
//! queens on an `n x n` board (board layouts themselves are skipped here
//! for a simpler, still-meaningful check).

pub fn total_n_queens(n: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_queens() {
        assert_eq!(total_n_queens(4), 2);
    }

    #[test]
    fn one_queen() {
        assert_eq!(total_n_queens(1), 1);
    }
}
