//! https://leetcode.com/problems/longest-common-subsequence/
//!
//! Return the length of the longest subsequence common to both `text1` and
//! `text2`.

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn no_common() {
        assert_eq!(longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
    }
}
