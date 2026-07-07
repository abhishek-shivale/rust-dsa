//! https://leetcode.com/problems/merge-sorted-array/
//!
//! `nums1` has length `m + n`, its first `m` entries sorted and the rest
//! zeroed as scratch space. Merge `nums2` (length `n`) in, sorted, in place.

pub fn merge(mut nums1: Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) -> Vec<i32> {
    let _ = (&mut nums1, m, n);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleaved() {
        assert_eq!(
            merge(vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3),
            vec![1, 2, 2, 3, 5, 6]
        );
    }

    #[test]
    fn second_empty() {
        assert_eq!(merge(vec![1], 1, vec![], 0), vec![1]);
    }
}
