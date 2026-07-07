//! https://leetcode.com/problems/daily-temperatures/
//!
//! For each day, return how many days until a strictly warmer temperature;
//! `0` if none.

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
