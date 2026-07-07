//! https://leetcode.com/problems/combination-sum/
//!
//! `candidates` are distinct positive integers, reusable any number of
//! times. Return all unique combinations summing to `target`.

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for c in v.iter_mut() {
            c.sort();
        }
        v.sort();
        v
    }

    #[test]
    fn example_1() {
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(normalize(combination_sum(vec![2, 3, 6, 7], 7)), normalize(expected));
    }
}
