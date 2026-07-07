//! https://leetcode.com/problems/jump-game/
//!
//! `nums[i]` is the max jump length from index `i`. Return whether you can
//! reach the last index starting from index 0.

pub fn can_jump(nums: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reachable() {
        assert!(can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn stuck() {
        assert!(!can_jump(vec![3, 2, 1, 0, 4]));
    }
}
