//! https://leetcode.com/problems/k-closest-points-to-origin/
//!
//! Return the `k` points from `points` closest to `(0, 0)` by Euclidean
//! distance, in any order.

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_1() {
        let result: HashSet<Vec<i32>> = k_closest(vec![vec![1, 3], vec![-2, 2]], 1).into_iter().collect();
        assert_eq!(result, HashSet::from([vec![-2, 2]]));
    }
}
