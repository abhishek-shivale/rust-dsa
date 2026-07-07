//! https://leetcode.com/problems/palindromic-substrings/
//!
//! Return the number of palindromic substrings of `s` (counting distinct
//! positions, not distinct strings).

pub fn count_substrings(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(count_substrings("abc".to_string()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(count_substrings("aaa".to_string()), 6);
    }
}
