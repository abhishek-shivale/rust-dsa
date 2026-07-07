//! https://leetcode.com/problems/counting-bits/
//!
//! For every `i` in `0..=n`, return the number of `1` bits in `i`.

pub fn count_bits(n: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_to_two() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn up_to_five() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
