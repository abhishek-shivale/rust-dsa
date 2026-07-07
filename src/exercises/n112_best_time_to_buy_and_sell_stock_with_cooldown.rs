//! https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
//!
//! May trade unlimited times, but must wait one cooldown day after selling
//! before buying again (no holding more than one share). Return max profit.

pub fn max_profit(prices: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn single_day() {
        assert_eq!(max_profit(vec![1]), 0);
    }
}
