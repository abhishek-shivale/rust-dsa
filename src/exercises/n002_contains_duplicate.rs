//! https://leetcode.com/problems/contains-duplicate/
//!
//! Return `true` if any value appears at least twice in `nums`.

pub fn has_duplicate(nums: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_dupe() {
        assert!(has_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn all_distinct() {
        assert!(!has_duplicate(vec![1, 2, 3, 4]));
    }
}
