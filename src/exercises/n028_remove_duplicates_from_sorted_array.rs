//! https://leetcode.com/problems/remove-duplicates-from-sorted-array/
//!
//! Remove duplicates from sorted `nums` in place; return the deduplicated
//! prefix as a vec (length `k`) for easy testing.

pub fn remove_duplicates(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(remove_duplicates(vec![1, 1, 2]), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            remove_duplicates(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            vec![0, 1, 2, 3, 4]
        );
    }
}
