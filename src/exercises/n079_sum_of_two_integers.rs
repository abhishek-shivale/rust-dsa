//! https://leetcode.com/problems/sum-of-two-integers/
//!
//! Return `a + b` without using the `+` or `-` operators (bit tricks only).

pub fn get_sum(a: i32, b: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(get_sum(1, 2), 3);
    }

    #[test]
    fn with_negative() {
        assert_eq!(get_sum(2, 3), 5);
    }
}
