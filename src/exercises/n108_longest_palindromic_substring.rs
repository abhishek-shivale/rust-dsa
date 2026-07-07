//! https://leetcode.com/problems/longest-palindromic-substring/
//!
//! Return the longest palindromic substring of `s`.

pub fn longest_palindrome(s: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = longest_palindrome("babad".to_string());
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn even_length() {
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
    }
}
