//! https://leetcode.com/problems/longest-substring-without-repeating-characters/
//!
//! Return the length of the longest substring of `s` without repeating
//! characters.

pub fn length_of_longest_substring(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn all_same() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }
}
