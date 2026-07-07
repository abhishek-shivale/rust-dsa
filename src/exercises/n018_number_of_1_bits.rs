//! https://leetcode.com/problems/number-of-1-bits/
//!
//! Return the count of `1` bits (Hamming weight) in the binary
//! representation of `n`.

pub fn hamming_weight(n: u32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eleven() {
        assert_eq!(hamming_weight(0b1011), 3);
    }

    #[test]
    fn power_of_two() {
        assert_eq!(hamming_weight(0b10000000), 1);
    }
}
