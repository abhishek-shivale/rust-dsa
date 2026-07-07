//! https://leetcode.com/problems/climbing-stairs/
//!
//! Climbing `n` stairs, 1 or 2 steps at a time: how many distinct ways to
//! reach the top?

pub fn climb_stairs(n: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_steps() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn three_steps() {
        assert_eq!(climb_stairs(3), 3);
    }
}
