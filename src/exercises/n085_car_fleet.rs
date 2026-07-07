//! https://leetcode.com/problems/car-fleet/
//!
//! Cars at `position[i]` moving toward `target` at `speed[i]`. Cars that
//! catch up form one fleet moving at the slower speed. Return the number
//! of fleets that arrive.

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
    }
}
