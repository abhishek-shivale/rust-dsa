//! https://leetcode.com/problems/plus-one/
//!
//! `digits` is a large integer's digits, most significant first. Return the
//! digits of that integer plus one.

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_carry() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn all_nines() {
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
