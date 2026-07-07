//! https://leetcode.com/problems/top-k-frequent-elements/
//!
//! Return the `k` most frequent elements, in any order.

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_1() {
        let result: HashSet<i32> = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2).into_iter().collect();
        assert_eq!(result, HashSet::from([1, 2]));
    }

    #[test]
    fn single_element() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
