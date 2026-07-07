//! https://leetcode.com/problems/partition-equal-subset-sum/
//!
//! Return whether `nums` can be split into two subsets with equal sum.

pub fn can_partition(nums: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_split() {
        assert!(can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn cannot_split() {
        assert!(!can_partition(vec![1, 2, 3, 5]));
    }
}
