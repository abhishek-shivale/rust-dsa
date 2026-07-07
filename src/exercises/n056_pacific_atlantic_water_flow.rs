//! https://leetcode.com/problems/pacific-atlantic-water-flow/
//!
//! `heights[r][c]` is the height at that cell. Water flows to
//! equal-or-lower neighbors. Return every cell from which water can reach
//! both the Pacific (top/left edges) and Atlantic (bottom/right edges).

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_1() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let result: HashSet<Vec<i32>> = pacific_atlantic(heights).into_iter().collect();
        let expected: HashSet<Vec<i32>> = vec![
            vec![0, 4], vec![1, 3], vec![1, 4], vec![2, 2], vec![3, 0],
            vec![3, 1], vec![4, 0],
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
}
