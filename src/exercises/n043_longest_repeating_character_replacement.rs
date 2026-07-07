//! https://leetcode.com/problems/longest-repeating-character-replacement/
//!
//! You may replace up to `k` characters of `s`. Return the length of the
//! longest substring achievable containing a single repeated character.

pub fn character_replacement(s: String, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}
