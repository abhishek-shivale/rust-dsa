//! https://leetcode.com/problems/redundant-connection/
//!
//! `edges` describes a graph that was a tree plus one extra edge. Return
//! that extra edge (the last one in `edges` that creates a cycle).

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]]),
            vec![1, 4]
        );
    }
}
