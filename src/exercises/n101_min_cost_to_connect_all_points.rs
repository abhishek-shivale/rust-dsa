//! https://leetcode.com/problems/min-cost-to-connect-all-points/
//!
//! `points[i] = [x, y]`. Cost between two points is Manhattan distance.
//! Return the cost of the minimum spanning tree connecting all points.

pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
        assert_eq!(min_cost_connect_points(points), 20);
    }
}
