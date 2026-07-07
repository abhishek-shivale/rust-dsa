//! https://leetcode.com/problems/3sum/
//!
//! Return all unique triplets in `nums` that sum to zero.

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for t in v.iter_mut() {
            t.sort();
        }
        v.sort();
        v
    }

    #[test]
    fn example_1() {
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(normalize(three_sum(vec![-1, 0, 1, 2, -1, -4])), normalize(expected));
    }

    #[test]
    fn no_triplets() {
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }
}
