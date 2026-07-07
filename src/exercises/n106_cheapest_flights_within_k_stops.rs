//! https://leetcode.com/problems/cheapest-flights-within-k-stops/
//!
//! `flights[i] = [from, to, price]`. Return the cheapest price from `src`
//! to `dst` using at most `k` stops (`k + 1` edges), or -1 if impossible.

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(3, flights, 0, 2, 1), 200);
    }

    #[test]
    fn no_stops_allowed() {
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(3, flights, 0, 2, 0), 500);
    }
}
