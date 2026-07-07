//! https://leetcode.com/problems/house-robber-ii/
//!
//! Same as House Robber, but houses are arranged in a circle (first and
//! last are adjacent).

pub fn rob(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }
}
