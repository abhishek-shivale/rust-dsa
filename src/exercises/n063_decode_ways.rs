//! https://leetcode.com/problems/decode-ways/
//!
//! Digits `1..=26` map to letters `A..=Z`. Return the number of ways to
//! decode the digit string `s`.

pub fn num_decodings(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(num_decodings("12".to_string()), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(num_decodings("226".to_string()), 3);
    }
}
