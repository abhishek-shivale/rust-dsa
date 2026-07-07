//! https://leetcode.com/problems/search-insert-position/
//!
//! `nums` is sorted, distinct. Return the index of `target`, or the index
//! it would be inserted at to keep the array sorted.

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn insert_middle() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }
}
