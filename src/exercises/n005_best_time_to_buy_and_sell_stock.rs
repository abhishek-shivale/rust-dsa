//! https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
//!
//! `prices[i]` is the stock price on day `i`. Pick one buy day and one
//! later sell day to maximize profit; return 0 if no profit is possible.

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;
    
    for price in prices {
        min_price = min_price.min(price);
        max_profit = max_profit.max(price - min_price);
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profit_exists() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn no_profit() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
