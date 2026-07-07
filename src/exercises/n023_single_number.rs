//! https://leetcode.com/problems/single-number/
//!
//! Every element appears twice except one. Return that one (linear time,
//! constant space).

pub fn single_number(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
