//! https://leetcode.com/problems/coin-change-ii/
//!
//! Return the number of distinct combinations of `coins` (unlimited
//! supply) that add up to `amount`.

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(change(5, vec![1, 2, 5]), 4);
    }

    #[test]
    fn impossible() {
        assert_eq!(change(3, vec![2]), 0);
    }
}
