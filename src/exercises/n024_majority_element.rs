//! https://leetcode.com/problems/majority-element/
//!
//! Return the element appearing more than `n / 2` times (Boyer-Moore vote).

pub fn majority_element(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
