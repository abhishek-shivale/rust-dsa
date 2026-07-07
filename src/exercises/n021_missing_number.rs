//! https://leetcode.com/problems/missing-number/
//!
//! `nums` holds `n` distinct values from `0..=n`, one missing. Return it.

pub fn missing_number(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_middle() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn missing_last() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }
}
