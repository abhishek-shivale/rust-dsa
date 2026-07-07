//! https://leetcode.com/problems/gas-station/
//!
//! `gas[i]` fuel available at station `i`, `cost[i]` fuel to travel to the
//! next station. Return the starting station index that lets you complete
//! the circuit, or -1 if none exists (there's at most one answer).

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    }

    #[test]
    fn impossible() {
        assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}
