//! https://leetcode.com/problems/top-k-frequent-words/
//!
//! Return the `k` most frequent words, ordered by frequency (descending),
//! ties broken alphabetically.

pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ws(s: &[&str]) -> Vec<String> {
        s.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn example_1() {
        let words = ws(&["i", "love", "leetcode", "i", "love", "coding"]);
        assert_eq!(top_k_frequent(words, 2), ws(&["i", "love"]));
    }
}
