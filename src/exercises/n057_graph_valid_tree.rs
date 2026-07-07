//! https://leetcode.com/problems/graph-valid-tree/ (Premium; classic all the same)
//!
//! Given `n` nodes labeled `0..n` and a list of undirected `edges`, return
//! whether they form a valid tree (connected, no cycles).

pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_a_tree() {
        assert!(valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]));
    }

    #[test]
    fn has_cycle() {
        assert!(!valid_tree(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]));
    }
}
