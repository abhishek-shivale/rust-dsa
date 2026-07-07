//! https://leetcode.com/problems/reverse-bits/
//!
//! Reverse the bits of a 32-bit unsigned integer.

pub fn reverse_bits(x: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(reverse_bits(0b00000010100101000001111010011100), 0b00111001011110000010100101000000);
    }

    #[test]
    fn all_ones() {
        assert_eq!(reverse_bits(u32::MAX), u32::MAX);
    }
}
