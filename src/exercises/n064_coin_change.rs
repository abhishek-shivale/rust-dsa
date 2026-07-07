//! https://leetcode.com/problems/coin-change/
//!
//! Return the fewest coins from `coins` (unlimited supply of each) needed
//! to make `amount`, or -1 if impossible.

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn impossible() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }
}
