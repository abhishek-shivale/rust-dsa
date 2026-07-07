//! https://leetcode.com/problems/subsets-ii/
//!
//! `nums` may contain duplicates. Return all unique subsets, in any order.

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for s in v.iter_mut() {
            s.sort();
        }
        v.sort();
        v.dedup();
        v
    }

    #[test]
    fn example_1() {
        let expected = vec![
            vec![], vec![1], vec![1, 1], vec![1, 1, 2], vec![1, 2], vec![2],
        ];
        assert_eq!(normalize(subsets_with_dup(vec![1, 2, 2])), normalize(expected));
    }
}
