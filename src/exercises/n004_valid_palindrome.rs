//! https://leetcode.com/problems/valid-palindrome/
//!
//! Lowercase `s`, strip non-alphanumerics, return whether it reads the same
//! forwards and backwards.

pub fn is_palindrome(s: String) -> bool {
    let palindrome: Vec<char> = s.to_lowercase().chars().filter(|a| a.is_alphanumeric()).collect();

    for i in 0..(palindrome.len() / 2) {
        if palindrome[i] == palindrome[palindrome.len() - 1 - i] {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn not_palindrome() {
        assert!(!is_palindrome("race a car".to_string()));
    }
}
