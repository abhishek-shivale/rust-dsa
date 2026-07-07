//! https://leetcode.com/problems/encode-and-decode-strings/ (Premium)
//!
//! Design an algorithm to encode a list of strings into one string, and
//! decode it back into the original list (strings may contain any
//! character).

pub fn encode(strs: Vec<String>) -> String {
    todo!()
}

pub fn decode(s: String) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let strs = vec!["neet".to_string(), "code".to_string(), "love".to_string(), "you".to_string()];
        assert_eq!(decode(encode(strs.clone())), strs);
    }

    #[test]
    fn handles_empty_strings() {
        let strs = vec!["".to_string(), "a".to_string(), "".to_string()];
        assert_eq!(decode(encode(strs.clone())), strs);
    }
}
