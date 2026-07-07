//! https://leetcode.com/problems/clone-graph/
//!
//! Deep-copy a connected undirected graph given a reference to one node.
//! Represented here as an adjacency list (`graph[i]` = neighbors of node
//! `i`, 0-indexed) rather than LeetCode's `Node` struct, to keep the
//! exercise dependency-free; return the cloned adjacency list.

pub fn clone_graph(graph: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_graph() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(clone_graph(graph.clone()), graph);
    }

    #[test]
    fn single_node() {
        assert_eq!(clone_graph(vec![vec![]]), vec![vec![]]);
    }
}
