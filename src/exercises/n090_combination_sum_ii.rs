//! https://leetcode.com/problems/combination-sum-ii/
//!
//! `candidates` may contain duplicates, each usable at most once. Return
//! all unique combinations summing to `target`.

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        let expected = vec![
            vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6], vec![6, 1, 1],
        ];
        let expected: Vec<Vec<i32>> = expected.into_iter().collect();
        assert_eq!(
            normalize(combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)),
            normalize(expected)
        );
    }
}
